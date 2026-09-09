use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::{
    exo_user_gates, load_exo_exchange, load_exo_price_update, split_gross_native,
};
use crate::instructions::stablecoin_ops::{burn_tokens, transfer_pda};
use crate::oracle::oracle_event;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::ExchangeContext;
use hylo_core::fees::controller::FeeExtract;
use hylo_core::rebalance::mode::RebalanceMode;
use hylo_core::util::normalize_mint_exp;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct RedeemLevercoinExo<'info> {
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
        token::mint = levercoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_levercoin_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = collateral_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_collateral_ta: Account<'info, TokenAccount>,
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
    ctx: Context<RedeemLevercoinExo>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<RedeemLevercoinExoEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let exo_pair = ctx.accounts.exo_pair.load()?;

    require!(amount > 0, CoreError::ZeroAmount);
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
        exchange.rebalance_mode() != RebalanceMode::Depeg,
        ErrorCode::LevercoinRedeemDisabled
    );

    let redeemed = UFix64::<N6>::new(amount);
    require!(
        redeemed <= exchange.levercoin_supply()?,
        CoreError::ZeroAmount
    );
    let nav = exchange.levercoin_redeem_nav()?;
    let gross_n9 = exchange.exo_conversion().token_to_exo(redeemed, nav)?;
    require!(gross_n9 > UFix64::zero(), CoreError::ZeroAmount);
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = exchange.levercoin_redeem_fee(gross_n9)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    let (fee_native, net_native) =
        split_gross_native(&ctx.accounts.collateral_mint, gross_n9, fees_extracted)?;
    require!(
        fee_native.saturating_add(net_native) <= ctx.accounts.collateral_vault.amount,
        CoreError::InsufficientLiquidity
    );
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out_normalized(&ctx.accounts.collateral_mint, amount_remaining)?;
    }

    let mint_key = ctx.accounts.collateral_mint.key();
    let vault_bump = [exo_pair.vault_auth_bump];
    let vault_seeds: &[&[u8]] = &[EXO_VAULT_AUTH, mint_key.as_ref(), &vault_bump];
    let decimals = ctx.accounts.collateral_mint.decimals;
    transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.collateral_vault.to_account_info(),
        ctx.accounts.collateral_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.vault_auth.to_account_info(),
        fee_native,
        decimals,
        vault_seeds,
    )?;
    transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.collateral_vault.to_account_info(),
        ctx.accounts.collateral_mint.to_account_info(),
        ctx.accounts.user_collateral_ta.to_account_info(),
        ctx.accounts.vault_auth.to_account_info(),
        net_native,
        decimals,
        vault_seeds,
    )?;
    burn_tokens(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_mint.to_account_info(),
        ctx.accounts.user_levercoin_ta.to_account_info(),
        ctx.accounts.user.to_account_info(),
        redeemed.bits,
    )?;

    let net_n9 = normalize_mint_exp(&ctx.accounts.collateral_mint, net_native)?;
    let event = RedeemLevercoinExoEvent {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        redeemed: redeemed.into(),
        nav: nav.into(),
        collateral_usd_price: oracle_event(exchange.collateral_oracle_price()),
        collateral_withdrawn: net_n9.into(),
        fees_deposited: fees_extracted.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
