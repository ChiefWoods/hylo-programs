use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::fees::controller::FeeExtract;
use hylo_core::pyth::SOL_USD;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::lst_user_gates;
use crate::instructions::stablecoin_ops::{burn_tokens, mint_stablecoin};
use crate::oracle::load_price_update;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct ConvertLeverToStableLst<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = stablecoin_mint,
        has_one = levercoin_mint,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.stablecoin_auth_bump,
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
    #[account(mut, seeds = [XSOL], bump = hylo.levercoin_mint_bump)]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump = hylo.levercoin_auth_bump,
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
    ctx: Context<ConvertLeverToStableLst>,
    amount_levercoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<ConvertLeverToStableLstEvent> {
    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    require!(amount_levercoin > 0, CoreError::ZeroAmount);
    let clock = Clock::get()?;
    lst_user_gates(&ctx.accounts.hylo, clock.epoch)?;

    let price_update = load_price_update(&ctx.accounts.sol_usd_pyth_feed, &SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &ctx.accounts.hylo.total_sol_cache,
        ctx.accounts.hylo.stablecoin_mint_threshold()?,
        ctx.accounts.hylo.oracle_config()?,
        ctx.accounts.hylo.levercoin_fees,
        &price_update,
        ctx.accounts.hylo.virtual_stablecoin,
        Some(&ctx.accounts.levercoin_mint),
        ctx.accounts.hylo.lst_sell_curve_config,
        ctx.accounts.hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.rebalance_mode() != hylo_core::rebalance::mode::RebalanceMode::Depeg,
        ErrorCode::LeverToStableDisabled
    );

    let burned = UFix64::<N6>::new(amount_levercoin);
    require!(burned <= exchange.levercoin_supply()?, CoreError::ZeroAmount);
    let levercoin_nav = exchange.levercoin_redeem_nav()?;
    let stablecoin_nav = exchange.stablecoin_nav()?;
    let gross = exchange
        .swap_conversion()?
        .lever_to_stable(burned)?
        .min(exchange.max_swappable_stablecoin()?);
    require!(gross > UFix64::zero(), CoreError::ZeroAmount);
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = exchange.levercoin_to_stablecoin_fee(gross)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(amount_remaining)?;
    }

    burn_tokens(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_mint.to_account_info(),
        ctx.accounts.user_levercoin_ta.to_account_info(),
        ctx.accounts.user.to_account_info(),
        burned.bits,
    )?;
    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.stablecoin_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        ctx.accounts.hylo.stablecoin_auth_bump,
        fees_extracted.bits,
    )?;
    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.stablecoin_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        ctx.accounts.hylo.stablecoin_auth_bump,
        amount_remaining.bits,
    )?;

    ctx.accounts.hylo.virtual_stablecoin.mint(gross)?;

    let event = ConvertLeverToStableLstEvent {
        levercoin_burned: burned.into(),
        levercoin_nav: levercoin_nav.into(),
        stablecoin_minted_user: amount_remaining.into(),
        stablecoin_minted_fees: fees_extracted.into(),
        stablecoin_nav: stablecoin_nav.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
