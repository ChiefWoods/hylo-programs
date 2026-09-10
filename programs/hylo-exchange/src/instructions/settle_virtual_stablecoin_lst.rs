use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedSub, UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::pyth::SOL_USD;
use hylo_core::virtual_stablecoin::SUPPLY_FLOOR;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG};
use crate::instructions::stablecoin_ops::{absorb_loss, drawdown_repay, mint_stablecoin};
use crate::oracle::load_price_update;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SettleVirtualStablecoinLst<'info> {
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [SETTLEMENT_AUTH], bump)]
    pub settlement_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump = pool_config.load()?.pool_auth_bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<SettleVirtualStablecoinLst>,
) -> Result<SettleVirtualStablecoinLstEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let clock = Clock::get()?;
    require!(
        hylo.yield_harvest_cache.epoch == clock.epoch,
        CoreError::YieldHarvestNotRun
    );
    let price_update = load_price_update(&ctx.accounts.sol_usd_pyth_feed, &SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &hylo.total_sol_cache,
        hylo.stablecoin_mint_threshold()?,
        hylo.oracle_config()?,
        hylo.levercoin_fees,
        &price_update,
        hylo.virtual_stablecoin,
        None,
        hylo.lst_sell_curve_config,
        hylo.lst_buy_curve_config,
    )?;

    let tvl = exchange
        .total_value_locked()?
        .checked_convert::<N6>()
        .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinConversion))?;
    let virtual_supply = hylo.virtual_stablecoin.supply()?;

    let (stablecoin_burned, stablecoin_minted) = if tvl > virtual_supply {
        let surplus = tvl
            .checked_sub(&virtual_supply)
            .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinUnderflow))?;
        require!(
            surplus > UFix64::zero(),
            ErrorCode::SettleVirtualStablecoinNoop
        );
        mint_stablecoin(
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.stablecoin_mint.to_account_info(),
            ctx.accounts.stablecoin_pool.to_account_info(),
            ctx.accounts.stablecoin_mint_auth.to_account_info(),
            ctx.accounts.stablecoin_mint.key(),
            hylo.stablecoin_auth_bump,
            surplus.bits,
        )?;
        hylo.virtual_stablecoin.mint(surplus)?;
        drawdown_repay(&mut hylo.pool_drawdown, surplus)?;
        (UFix64::zero(), surplus)
    } else if virtual_supply > tvl {
        let overhang = virtual_supply
            .checked_sub(&tvl)
            .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinUnderflow))?;
        let max_burn = virtual_supply
            .checked_sub(&SUPPLY_FLOOR)
            .unwrap_or_else(UFix64::zero);
        let pool = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
        let burned = overhang.min(max_burn).min(pool);
        require!(
            burned > UFix64::zero(),
            ErrorCode::SettleVirtualStablecoinNoop
        );
        drop(hylo);
        absorb_loss(
            ctx.accounts.earn_pool.to_account_info(),
            ctx.accounts.settlement_auth.to_account_info(),
            ctx.accounts.hylo.to_account_info(),
            ctx.accounts.pool_config.to_account_info(),
            ctx.accounts.pool_auth.to_account_info(),
            ctx.accounts.stablecoin_pool.to_account_info(),
            ctx.accounts.stablecoin_mint.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.bumps.settlement_auth,
            burned.bits,
        )?;
        hylo = ctx.accounts.hylo.load_mut()?;
        hylo.virtual_stablecoin.burn_limited(burned, SUPPLY_FLOOR)?;
        hylo.pool_drawdown.drawdown(burned)?;
        (burned, UFix64::zero())
    } else {
        return err!(ErrorCode::SettleVirtualStablecoinNoop);
    };

    let pool_balance = if stablecoin_minted > UFix64::zero() {
        ctx.accounts
            .stablecoin_pool
            .amount
            .saturating_add(stablecoin_minted.bits)
    } else {
        ctx.accounts
            .stablecoin_pool
            .amount
            .saturating_sub(stablecoin_burned.bits)
    };

    let event = SettleVirtualStablecoinLstEvent {
        stablecoin_burned: stablecoin_burned.into(),
        stablecoin_minted: stablecoin_minted.into(),
        virtual_stablecoin_supply: hylo.virtual_stablecoin.supply()?.into(),
        pool_drawdown_outstanding: hylo.pool_drawdown.outstanding()?.into(),
        pool_balance: UFix64::<N6>::new(pool_balance).into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
