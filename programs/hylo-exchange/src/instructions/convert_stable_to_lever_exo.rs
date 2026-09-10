use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::ExchangeContext;
use hylo_core::fees::controller::FeeExtract;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::{exo_user_gates, load_exo_exchange, load_exo_price_update};
use crate::instructions::stablecoin_ops::{burn_tokens, mint_stablecoin, transfer_user};
use crate::oracle::oracle_event;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct ConvertStableToLeverExo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = stablecoin_mint,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
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
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.vault_auth_bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_levercoin_ta: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.levercoin_mint_bump,
    )]
    pub levercoin_mint: Box<Account<'info, Mint>>,
    pub collateral_mint: Box<Account<'info, Mint>>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ConvertStableToLeverExo>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<ConvertStableToLeverExoEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

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
        Some(ctx.accounts.levercoin_mint.as_ref().as_ref()),
    )?;
    require!(
        exchange.levercoin_mint_enabled(),
        ErrorCode::StableToLeverDisabled
    );

    let amount_in = UFix64::<N6>::new(amount);
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = exchange.stablecoin_to_levercoin_fee(amount_in)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    let stablecoin_nav = exchange.stablecoin_nav()?;
    let levercoin_nav = exchange.levercoin_mint_nav()?;
    let minted = exchange
        .swap_conversion()?
        .stable_to_lever(amount_remaining)?;
    require!(minted > UFix64::zero(), CoreError::ZeroAmount);
    exchange
        .levercoin_market_cap_limiter()?
        .validate_token_out(minted)?;
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(minted)?;
    }

    let decimals = ctx.accounts.stablecoin_mint.decimals;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        fees_extracted.bits,
        decimals,
    )?;
    burn_tokens(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.user.to_account_info(),
        amount_remaining.bits,
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

    let floor = exo_pair.virtual_stablecoin_supply_floor.try_into()?;
    exo_pair
        .virtual_stablecoin
        .burn_limited(amount_remaining, floor)?;

    let event = ConvertStableToLeverExoEvent {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        stablecoin_burned: amount_remaining.into(),
        stablecoin_fees: fees_extracted.into(),
        stablecoin_nav: stablecoin_nav.into(),
        levercoin_minted: minted.into(),
        levercoin_nav: levercoin_nav.into(),
        collateral_usd_price: oracle_event(exchange.collateral_oracle_price()),
        virtual_stablecoin_supply: exo_pair.virtual_stablecoin.supply()?.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
