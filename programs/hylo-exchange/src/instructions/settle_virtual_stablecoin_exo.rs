use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedSub, UFix64, N6};
use hylo_core::exchange_context::{ExchangeContext, ExoExchangeContext};
use hylo_core::util::normalize_mint_exp;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG};
use crate::instructions::stablecoin_ops::{absorb_loss, drawdown_repay, mint_stablecoin};
use crate::oracle::load_price_update;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SettleVirtualStablecoinExo<'info> {
    #[account(seeds = [HYLO], bump)]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
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
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.vault_auth_bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<SettleVirtualStablecoinExo>,
) -> Result<SettleVirtualStablecoinExoEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

    require_keys_eq!(
        ctx.accounts.collateral_usd_pyth_feed.key(),
        exo_pair.oracle,
        ErrorCode::ExoOracleInvalid
    );

    let clock = Clock::get()?;
    let price_update = load_price_update(
        &ctx.accounts.collateral_usd_pyth_feed,
        &exo_pair.oracle_feed_id,
    )
    .map_err(|_| error!(ErrorCode::ExoOracleInvalid))?;
    let total_collateral = normalize_mint_exp(
        &ctx.accounts.collateral_mint,
        ctx.accounts.collateral_vault.amount,
    )?;
    let exchange = ExoExchangeContext::load(
        clock,
        total_collateral,
        exo_pair.stablecoin_mint_threshold()?,
        exo_pair.oracle_config()?,
        exo_pair.levercoin_fees,
        &price_update,
        exo_pair.virtual_stablecoin,
        None,
        exo_pair.sell_curve_config,
        exo_pair.buy_curve_config,
        exo_pair.levercoin_market_cap_limit.try_into()?,
    )?;

    let tvl = exchange
        .total_value_locked()?
        .checked_convert::<N6>()
        .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinConversion))?;
    let virtual_supply = exo_pair.virtual_stablecoin.supply()?;
    let floor: UFix64<N6> = exo_pair.virtual_stablecoin_supply_floor.try_into()?;

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
        exo_pair.virtual_stablecoin.mint(surplus)?;
        drawdown_repay(&mut exo_pair.pool_drawdown, surplus)?;
        (UFix64::zero(), surplus)
    } else if virtual_supply > tvl {
        let overhang = virtual_supply
            .checked_sub(&tvl)
            .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinUnderflow))?;
        let max_burn = virtual_supply
            .checked_sub(&floor)
            .unwrap_or_else(UFix64::zero);
        let pool = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
        let burned = overhang.min(max_burn).min(pool);
        require!(
            burned > UFix64::zero(),
            ErrorCode::SettleVirtualStablecoinNoop
        );
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
        exo_pair.virtual_stablecoin.burn_limited(burned, floor)?;
        exo_pair.pool_drawdown.drawdown(burned)?;
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

    let event = SettleVirtualStablecoinExoEvent {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        stablecoin_burned: stablecoin_burned.into(),
        stablecoin_minted: stablecoin_minted.into(),
        virtual_stablecoin_supply: exo_pair.virtual_stablecoin.supply()?.into(),
        pool_drawdown_outstanding: exo_pair.pool_drawdown.outstanding()?.into(),
        pool_balance: UFix64::<N6>::new(pool_balance).into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
