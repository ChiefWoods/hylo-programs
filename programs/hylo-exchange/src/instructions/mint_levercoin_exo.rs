use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::{
    exo_user_gates, load_exo_exchange, load_exo_price_update, split_collateral_native,
};
use crate::instructions::stablecoin_ops::{mint_stablecoin, transfer_user};
use crate::oracle::oracle_event;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::UFix64;
use hylo_core::error::CoreError;
use hylo_core::exchange_context::ExchangeContext;
use hylo_core::fees::controller::FeeExtract;
use hylo_core::util::normalize_mint_exp;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct MintLevercoinExo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump = exo_pair.load()?.levercoin_auth_bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.vault_auth_bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.fee_auth_bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = collateral_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_collateral_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_levercoin_ta: Account<'info, TokenAccount>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.levercoin_mint_bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<MintLevercoinExo>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<MintLevercoinExoEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let exo_pair = ctx.accounts.exo_pair.load()?;

    require!(amount > 0, CoreError::ZeroAmount);
    require!(
        ctx.accounts.levercoin_mint.supply > 0,
        ErrorCode::ExoGenesisConstraints
    );
    let clock = Clock::get()?;
    exo_user_gates(&hylo, &exo_pair, clock.epoch)?;
    let price_update = load_exo_price_update(&ctx.accounts.collateral_usd_pyth_feed, &exo_pair)?;
    let exchange = load_exo_exchange(
        clock,
        &exo_pair,
        &ctx.accounts.collateral_mint,
        &ctx.accounts.collateral_vault,
        &price_update,
        Some(&ctx.accounts.levercoin_mint),
    )?;
    require!(
        exchange.levercoin_mint_enabled(),
        ErrorCode::LevercoinMintDisabled
    );

    let amount_n9 = normalize_mint_exp(&ctx.accounts.collateral_mint, amount)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    let FeeExtract {
        fees_extracted,
        amount_remaining: _,
    } = exchange.levercoin_mint_fee(amount_n9)?;
    let (fee_native, net_native, net_n9) =
        split_collateral_native(&ctx.accounts.collateral_mint, amount, fees_extracted)?;
    require!(net_native > 0, CoreError::ZeroAmount);
    let nav = exchange.levercoin_mint_nav()?;
    let minted = exchange.exo_conversion().exo_to_token(net_n9, nav)?;
    require!(minted > UFix64::zero(), CoreError::ZeroAmount);
    exchange
        .levercoin_market_cap_limiter()?
        .validate_token_out(minted)?;
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(minted)?;
    }

    let decimals = ctx.accounts.collateral_mint.decimals;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_collateral_ta.to_account_info(),
        ctx.accounts.collateral_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        fee_native,
        decimals,
    )?;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_collateral_ta.to_account_info(),
        ctx.accounts.collateral_mint.to_account_info(),
        ctx.accounts.collateral_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        net_native,
        decimals,
    )?;
    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_mint.to_account_info(),
        ctx.accounts.user_levercoin_ta.to_account_info(),
        ctx.accounts.levercoin_auth.to_account_info(),
        ctx.accounts.levercoin_mint.key(),
        exo_pair.levercoin_auth_bump,
        minted.bits,
    )?;

    let event = MintLevercoinExoEvent {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        minted: minted.into(),
        nav: nav.into(),
        oracle: exo_pair.oracle,
        collateral_usd_price: oracle_event(exchange.collateral_oracle_price()),
        collateral_deposited: net_n9.into(),
        fees_deposited: fees_extracted.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
