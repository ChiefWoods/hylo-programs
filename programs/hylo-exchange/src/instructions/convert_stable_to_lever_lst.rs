use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::fees::controller::FeeExtract;
use hylo_core::pyth::SOL_USD;
use hylo_core::virtual_stablecoin::SUPPLY_FLOOR;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::lst_user_gates;
use crate::instructions::stablecoin_ops::{burn_tokens, mint_stablecoin, transfer_user};
use crate::oracle::load_price_update;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct ConvertStableToLeverLst<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = stablecoin_mint,
        has_one = levercoin_mint,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Account<'info, TokenAccount>,
    #[account(mut, seeds = [XSOL], bump = hylo.load()?.levercoin_mint_bump)]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump = hylo.load()?.levercoin_auth_bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_levercoin_ta: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ConvertStableToLeverLst>,
    amount_stablecoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<ConvertStableToLeverLstEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    require!(amount_stablecoin > 0, CoreError::ZeroAmount);
    let clock = Clock::get()?;
    lst_user_gates(&hylo, clock.epoch)?;

    let price_update = load_price_update(&ctx.accounts.sol_usd_pyth_feed, &SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &hylo.total_sol_cache,
        hylo.stablecoin_mint_threshold()?,
        hylo.oracle_config()?,
        hylo.levercoin_fees,
        &price_update,
        hylo.virtual_stablecoin,
        Some(&ctx.accounts.levercoin_mint),
        hylo.lst_sell_curve_config,
        hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.levercoin_mint_enabled(),
        ErrorCode::StableToLeverDisabled
    );

    let amount_in = UFix64::<N6>::new(amount_stablecoin);
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
        hylo.levercoin_auth_bump,
        minted.bits,
    )?;

    hylo.virtual_stablecoin
        .burn_limited(amount_remaining, SUPPLY_FLOOR)?;

    let event = ConvertStableToLeverLstEvent {
        stablecoin_burned: amount_remaining.into(),
        stablecoin_fees: fees_extracted.into(),
        stablecoin_nav: stablecoin_nav.into(),
        levercoin_minted: minted.into(),
        levercoin_nav: levercoin_nav.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
