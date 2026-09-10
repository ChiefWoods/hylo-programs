use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedAdd, CheckedSub, UFix64, N4, N6};
use fix::typenum::Z0;
use hylo_core::exchange_context::{ExchangeContext, ExoExchangeContext};
use hylo_core::fees::controller::FeeExtract;
use hylo_core::rebalance::mode::RebalanceMode;
use hylo_core::util::normalize_mint_exp;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::stablecoin_ops::{drawdown_repay, mint_stablecoin};
use crate::oracle::{load_price_update, oracle_event};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct HarvestBorrowRate<'info> {
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
    pub stablecoin_fee_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = stablecoin_fee_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_fee_vault: Box<Account<'info, TokenAccount>>,
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Box<Account<'info, Mint>>,
    #[account(seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()], bump = exo_pair.load()?.levercoin_mint_bump)]
    pub levercoin_mint: Box<Account<'info, Mint>>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub hylo_earn_pool: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<HarvestBorrowRate>) -> Result<HarvestBorrowRateEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

    require_keys_eq!(
        ctx.accounts.collateral_usd_pyth_feed.key(),
        exo_pair.oracle,
        ErrorCode::ExoOracleInvalid
    );

    let clock = Clock::get()?;
    let epoch = clock.epoch;
    require!(
        exo_pair.borrow_rate_harvest_cache.is_stale(epoch),
        ErrorCode::BorrowRateHarvestAlreadyRun
    );
    let elapsed = epoch
        .checked_sub(exo_pair.borrow_rate_harvest_cache.epoch)
        .ok_or_else(|| error!(ErrorCode::BorrowRateHarvestEpochUnderflow))?;

    let price_update = load_price_update(
        &ctx.accounts.collateral_usd_pyth_feed,
        &exo_pair.oracle_feed_id,
    )
    .map_err(|_| error!(ErrorCode::ExoOracleInvalid))?;
    let total_collateral = normalize_mint_exp(
        &ctx.accounts.collateral_mint,
        ctx.accounts.collateral_vault.amount,
    )
    .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    let exchange = ExoExchangeContext::load(
        clock,
        total_collateral,
        exo_pair.stablecoin_mint_threshold()?,
        exo_pair.oracle_config()?,
        exo_pair.levercoin_fees,
        &price_update,
        exo_pair.virtual_stablecoin,
        Some(ctx.accounts.levercoin_mint.as_ref().as_ref()),
        exo_pair.sell_curve_config,
        exo_pair.buy_curve_config,
        exo_pair.levercoin_market_cap_limit.try_into()?,
    )?;

    let levercoin_market_cap = exchange.levercoin_market_cap()?;
    let gross_n9 = if exchange.rebalance_mode() < RebalanceMode::Neutral {
        UFix64::zero()
    } else {
        exo_pair.borrow_rate_curve_config.apply_borrow_rate(
            levercoin_market_cap,
            exchange.collateral_ratio(),
            UFix64::<Z0>::new(elapsed),
        )?
    };
    let gross: UFix64<N6> = gross_n9
        .checked_convert()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))?;
    let fee: UFix64<N4> = exo_pair.borrow_rate_fee.try_into()?;
    let extract = FeeExtract::new(fee, gross)?;

    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.stablecoin_fee_vault.to_account_info(),
        ctx.accounts.stablecoin_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        hylo.stablecoin_auth_bump,
        extract.fees_extracted.bits,
    )?;
    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.stablecoin_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        hylo.stablecoin_auth_bump,
        extract.amount_remaining.bits,
    )?;

    let minted = extract
        .fees_extracted
        .checked_add(&extract.amount_remaining)
        .ok_or_else(|| error!(ErrorCode::LstAdditionOverflow))?;
    if minted > UFix64::zero() {
        exo_pair.virtual_stablecoin.mint(minted)?;
    }
    let pool_drawdown_repaid =
        drawdown_repay(&mut exo_pair.pool_drawdown, extract.amount_remaining)?;
    let net_to_pool = extract
        .amount_remaining
        .checked_sub(&pool_drawdown_repaid)
        .unwrap_or_else(UFix64::zero);

    let pool_balance = UFix64::<N6>::new(
        ctx.accounts
            .stablecoin_pool
            .amount
            .saturating_add(extract.amount_remaining.bits),
    );
    exo_pair
        .borrow_rate_harvest_cache
        .update(pool_balance, net_to_pool, epoch)?;

    let event = HarvestBorrowRateEvent {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        levercoin_market_cap: levercoin_market_cap.into(),
        total_stablecoin_harvested: gross.into(),
        fees_extracted: extract.fees_extracted.into(),
        stablecoin_to_pool: extract.amount_remaining.into(),
        pool_drawdown_repaid: pool_drawdown_repaid.into(),
        collateral_usd_price: oracle_event(exchange.collateral_oracle_price()),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
