//! Shared rebalance-swap quoting, token CPIs, and PnL settlement.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, TransferChecked};
use fix::prelude::{CheckedAdd, CheckedSub, UFix64, N6, N9};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, ExoExchangeContext, LstExchangeContext};
use hylo_core::lst::stake_pool::SplStakePool;
use hylo_core::pyth::{query_pyth_oracle, OraclePrice, USDC_USD};
use hylo_core::rebalance::mode::RebalanceMode;
use hylo_core::rebalance::pnl::RebalancePnl;
use hylo_core::rebalance::pool_drawdown::PoolDrawdown;
use hylo_core::rebalance::pricing::RebalancePriceController;
use hylo_core::solana_clock::SolanaClock;
use hylo_core::util::{denormalize_mint_exp, normalize_mint_exp};
use hylo_core::virtual_stablecoin::{VirtualStablecoin, SUPPLY_FLOOR};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_earn_pool::{self, accounts::PoolConfig};
use crate::oracle::{load_price_update, oracle_event};
use crate::{events::*, state::*};

pub struct LstUsdcAccounts<'a, 'info> {
    pub user: &'a Signer<'info>,
    pub hylo: &'a AccountLoader<'info, Hylo>,
    pub pool_config: &'a AccountLoader<'info, PoolConfig>,
    pub lst_header: &'a AccountLoader<'info, LstHeader>,
    pub pool_state: &'a UncheckedAccount<'info>,
    pub usdc_pair: &'a AccountLoader<'info, UsdcPair>,
    pub stablecoin_mint_auth: &'a UncheckedAccount<'info>,
    pub lst_vault_auth: &'a UncheckedAccount<'info>,
    pub usdc_vault_auth: &'a UncheckedAccount<'info>,
    pub pool_auth: &'a UncheckedAccount<'info>,
    pub settlement_auth: &'a UncheckedAccount<'info>,
    pub lst_vault: &'a mut Account<'info, TokenAccount>,
    pub usdc_vault: &'a mut Account<'info, TokenAccount>,
    pub stablecoin_pool: &'a mut Account<'info, TokenAccount>,
    pub user_lst_ta: &'a mut Account<'info, TokenAccount>,
    pub user_usdc_ta: &'a mut Account<'info, TokenAccount>,
    pub lst_mint: &'a Account<'info, Mint>,
    pub usdc_mint: &'a Account<'info, Mint>,
    pub stablecoin_mint: &'a mut Account<'info, Mint>,
    pub sol_usd_pyth_feed: &'a UncheckedAccount<'info>,
    pub usdc_usd_pyth_feed: &'a UncheckedAccount<'info>,
    pub token_program: &'a Program<'info, Token>,
    pub earn_pool: &'a UncheckedAccount<'info>,
    pub lst_vault_auth_bump: u8,
    pub settlement_auth_bump: u8,
}

pub struct ExoUsdcAccounts<'a, 'info> {
    pub user: &'a Signer<'info>,
    pub hylo: &'a AccountLoader<'info, Hylo>,
    pub pool_config: &'a AccountLoader<'info, PoolConfig>,
    pub exo_pair: &'a AccountLoader<'info, ExoPair>,
    pub usdc_pair: &'a AccountLoader<'info, UsdcPair>,
    pub stablecoin_mint_auth: &'a UncheckedAccount<'info>,
    pub vault_auth: &'a UncheckedAccount<'info>,
    pub usdc_vault_auth: &'a UncheckedAccount<'info>,
    pub pool_auth: &'a UncheckedAccount<'info>,
    pub settlement_auth: &'a UncheckedAccount<'info>,
    pub collateral_vault: &'a mut Account<'info, TokenAccount>,
    pub usdc_collateral_vault: &'a mut Account<'info, TokenAccount>,
    pub stablecoin_pool: &'a mut Account<'info, TokenAccount>,
    pub user_collateral_ta: &'a mut Account<'info, TokenAccount>,
    pub user_usdc_ta: &'a mut Account<'info, TokenAccount>,
    pub collateral_mint: &'a Account<'info, Mint>,
    pub usdc_mint: &'a Account<'info, Mint>,
    pub stablecoin_mint: &'a mut Account<'info, Mint>,
    pub levercoin_mint: &'a Account<'info, Mint>,
    pub collateral_usd_pyth_feed: &'a UncheckedAccount<'info>,
    pub usdc_usd_pyth_feed: &'a UncheckedAccount<'info>,
    pub token_program: &'a Program<'info, Token>,
    pub earn_pool: &'a UncheckedAccount<'info>,
    pub settlement_auth_bump: u8,
}

pub(crate) fn assert_usdc_par(
    clock: &Clock,
    usdc_pair: &UsdcPair,
    feed: &UncheckedAccount,
) -> Result<OraclePrice> {
    let price_update = load_price_update(feed, &USDC_USD.feed_id)?;
    let oracle_price = query_pyth_oracle(clock, &price_update, usdc_pair.oracle_config()?)?;
    usdc_pair.par_tolerance.validate_spot(oracle_price.spot)?;
    Ok(oracle_price)
}

fn usdc_amount_n9(usdc: UFix64<N6>) -> Result<UFix64<N9>> {
    usdc.checked_convert()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))
}

fn lst_gates(hylo: &Hylo, usdc_pair: &UsdcPair, epoch: u64) -> Result<()> {
    require!(!hylo.protocol_paused, CoreError::ProtocolPaused);
    require!(!hylo.lst_pair_paused, CoreError::PairPaused);
    require!(!usdc_pair.paused, CoreError::PairPaused);
    require!(
        hylo.yield_harvest_cache.epoch == epoch,
        CoreError::YieldHarvestNotRun
    );
    Ok(())
}

fn exo_gates(hylo: &Hylo, exo_pair: &ExoPair, usdc_pair: &UsdcPair, epoch: u64) -> Result<()> {
    require!(!hylo.protocol_paused, CoreError::ProtocolPaused);
    require!(!exo_pair.paused, CoreError::PairPaused);
    require!(!usdc_pair.paused, CoreError::PairPaused);
    require!(
        exo_pair.borrow_rate_harvest_cache.epoch == epoch,
        CoreError::BorrowRateHarvestNotRun
    );
    Ok(())
}

fn adjusted_lst_price(
    header: &LstHeader,
    pool_state: &UncheckedAccount,
) -> Result<hylo_core::lst::sol_price::LstSolPrice> {
    let true_price = SplStakePool::from_bytes(&pool_state.try_borrow_data()?)?.true_price()?;
    Ok(true_price.adjust_price(header.rebalance_fee()?)?)
}

fn transfer_checked_user<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    token::transfer_checked(
        CpiContext::new(
            token_program,
            TransferChecked {
                from,
                mint,
                to,
                authority,
            },
        ),
        amount,
        decimals,
    )
}

fn transfer_checked_pda<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    token::transfer_checked(
        CpiContext::new_with_signer(
            token_program,
            TransferChecked {
                from,
                mint,
                to,
                authority,
            },
            &[signer_seeds],
        ),
        amount,
        decimals,
    )
}

struct PnlCpi<'info> {
    hylo: AccountInfo<'info>,
    pool_config: AccountInfo<'info>,
    pool_auth: AccountInfo<'info>,
    settlement_auth: AccountInfo<'info>,
    stablecoin_pool: AccountInfo<'info>,
    stablecoin_mint: AccountInfo<'info>,
    stablecoin_mint_auth: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    earn_pool: AccountInfo<'info>,
    stablecoin_mint_key: Pubkey,
    stablecoin_auth_bump: u8,
    settlement_auth_bump: u8,
}

fn settle_pnl<'info>(
    pnl: RebalancePnl,
    virtual_stablecoin: &mut VirtualStablecoin,
    pool_drawdown: &mut PoolDrawdown,
    _floor: UFix64<N6>,
    mode: RebalanceMode,
    pool_balance: u64,
    cpi: PnlCpi<'info>,
) -> Result<(UFix64<N6>, UFix64<N6>)> {
    require!(
        mode != RebalanceMode::Depeg,
        ErrorCode::SettleRebalancePnlDisabled
    );
    match pnl {
        RebalancePnl::NoChange => Ok((UFix64::zero(), UFix64::zero())),
        RebalancePnl::Profit(profit) => {
            require!(profit > UFix64::zero(), CoreError::ZeroAmount);
            let stablecoin_mint_key = cpi.stablecoin_mint_key;
            let auth_bump = [cpi.stablecoin_auth_bump];
            let mint_seeds: &[&[u8]] = &[MINT_AUTH, stablecoin_mint_key.as_ref(), &auth_bump];
            token::mint_to(
                CpiContext::new_with_signer(
                    cpi.token_program.clone(),
                    MintTo {
                        mint: cpi.stablecoin_mint.clone(),
                        to: cpi.stablecoin_pool.clone(),
                        authority: cpi.stablecoin_mint_auth.clone(),
                    },
                    &[mint_seeds],
                ),
                profit.bits,
            )?;
            let repay = profit.min(pool_drawdown.outstanding()?);
            if repay > UFix64::zero() {
                pool_drawdown.repay(repay)?;
            }
            virtual_stablecoin.mint(profit)?;
            Ok((UFix64::zero(), profit))
        }
        RebalancePnl::Loss(loss) => {
            require!(loss > UFix64::zero(), CoreError::ZeroAmount);
            require!(
                UFix64::<N6>::new(pool_balance) >= loss,
                CoreError::InsufficientEarnPoolLiquidity
            );
            let settlement_bump = [cpi.settlement_auth_bump];
            let settlement_seeds: &[&[u8]] = &[SETTLEMENT_AUTH, &settlement_bump];
            hylo_earn_pool::cpi::absorb_loss(
                CpiContext::new_with_signer(
                    cpi.earn_pool.clone(),
                    hylo_earn_pool::cpi::accounts::AbsorbLoss {
                        settlement_auth: cpi.settlement_auth.clone(),
                        hylo: cpi.hylo.clone(),
                        pool_config: cpi.pool_config.clone(),
                        pool_auth: cpi.pool_auth.clone(),
                        stablecoin_pool: cpi.stablecoin_pool.clone(),
                        stablecoin_mint: cpi.stablecoin_mint.clone(),
                        token_program: cpi.token_program.clone(),
                    },
                    &[settlement_seeds],
                ),
                loss.bits,
            )?;
            Ok((loss, UFix64::zero()))
        }
    }
}

fn apply_rebalance_loss(
    virtual_stablecoin: &mut VirtualStablecoin,
    pool_drawdown: &mut PoolDrawdown,
    loss: UFix64<N6>,
    floor: UFix64<N6>,
) -> Result<()> {
    if loss > UFix64::zero() {
        virtual_stablecoin.burn_limited(loss, floor)?;
        pool_drawdown.drawdown(loss)?;
    }
    Ok(())
}

fn pnl_cpi_from_lst<'info>(
    a: &LstUsdcAccounts<'_, 'info>,
    stablecoin_auth_bump: u8,
) -> Result<PnlCpi<'info>> {
    Ok(PnlCpi {
        hylo: a.hylo.to_account_info(),
        pool_config: a.pool_config.to_account_info(),
        pool_auth: a.pool_auth.to_account_info(),
        settlement_auth: a.settlement_auth.to_account_info(),
        stablecoin_pool: a.stablecoin_pool.to_account_info(),
        stablecoin_mint: a.stablecoin_mint.to_account_info(),
        stablecoin_mint_auth: a.stablecoin_mint_auth.to_account_info(),
        token_program: a.token_program.to_account_info(),
        earn_pool: a.earn_pool.to_account_info(),
        stablecoin_mint_key: a.stablecoin_mint.key(),
        stablecoin_auth_bump,
        settlement_auth_bump: a.settlement_auth_bump,
    })
}

/// `amount == None` uses the user's full LST balance, capped by buy-side capacity.
#[inline(never)]
pub fn swap_lst_to_usdc(
    a: LstUsdcAccounts,
    amount: Option<u64>,
    slippage_config: Option<SlippageConfig>,
) -> Result<(SwapLstToUsdcEvent, SettleRebalancePnlLstEvent)> {
    let mut hylo = a.hylo.load_mut()?;
    let mut usdc_pair = a.usdc_pair.load_mut()?;
    let lst_header = a.lst_header.load()?;
    let clock = Clock::get()?;
    lst_gates(&hylo, &usdc_pair, clock.epoch())?;
    let usdc_oracle = assert_usdc_par(&clock, &usdc_pair, a.usdc_usd_pyth_feed)?;

    let epoch = clock.epoch();
    let sol_price_update =
        load_price_update(a.sol_usd_pyth_feed, &hylo_core::pyth::SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &hylo.total_sol_cache,
        hylo.stablecoin_mint_threshold()?,
        hylo.oracle_config()?,
        hylo.levercoin_fees,
        &sol_price_update,
        hylo.virtual_stablecoin,
        None,
        hylo.lst_sell_curve_config,
        hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.rebalance_buy_active(),
        ErrorCode::RebalanceBuyInactive
    );

    let adjusted = adjusted_lst_price(&lst_header, a.pool_state)?;
    let buy_target_lst = adjusted.convert_sol_to_lst(exchange.rebalance_buy_target()?, epoch)?;
    let requested = match amount {
        Some(raw) => {
            require!(raw > 0, CoreError::ZeroAmount);
            let lst_in = UFix64::<N9>::new(raw);
            require!(
                lst_in <= buy_target_lst,
                CoreError::RebalanceBuyTargetExceeded
            );
            lst_in
        }
        None => {
            let balance = UFix64::<N9>::new(a.user_lst_ta.amount);
            require!(balance > UFix64::zero(), CoreError::ZeroAmount);
            balance.min(buy_target_lst)
        }
    };
    require!(requested > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        a.user_lst_ta.amount >= requested.bits,
        CoreError::InsufficientLiquidity
    );

    let mut requested = requested;
    let mut conversion = exchange.rebalance_buy_conversion(&adjusted, requested)?;
    let mut usdc_out = conversion.lst_to_token(requested, UFix64::one())?;
    if amount.is_none() && usdc_out.bits > a.usdc_vault.amount {
        requested = conversion.token_to_lst(UFix64::new(a.usdc_vault.amount), UFix64::one())?;
        require!(requested > UFix64::zero(), CoreError::ZeroAmount);
        conversion = exchange.rebalance_buy_conversion(&adjusted, requested)?;
        usdc_out = conversion.lst_to_token(requested, UFix64::one())?;
    }
    require!(usdc_out > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        usdc_out.bits <= a.usdc_vault.amount,
        CoreError::InsufficientLiquidity
    );
    usdc_pair
        .virtual_stablecoin
        .supply()?
        .checked_sub(&usdc_out)
        .ok_or(CoreError::BurnUnderflow)?;
    if let Some(slippage_config) = slippage_config.as_ref() {
        slippage_config.validate_token_out(usdc_out)?;
    }

    let pnl = exchange.rebalance_pnl_buy_side(&lst_header.price_sol, requested, usdc_out)?;
    let lst_sol_price = conversion.lst_sol_price;

    let usdc_mint_key = a.usdc_mint.key();
    let usdc_vault_bump = [usdc_pair.vault_auth_bump];
    let usdc_vault_seeds: &[&[u8]] = &[USDC_VAULT_AUTH, usdc_mint_key.as_ref(), &usdc_vault_bump];

    let lst_before = UFix64::<N9>::new(a.lst_vault.amount);
    transfer_checked_user(
        a.token_program.to_account_info(),
        a.user_lst_ta.to_account_info(),
        a.lst_mint.to_account_info(),
        a.lst_vault.to_account_info(),
        a.user.to_account_info(),
        requested.bits,
        a.lst_mint.decimals,
    )?;
    transfer_checked_pda(
        a.token_program.to_account_info(),
        a.usdc_vault.to_account_info(),
        a.usdc_mint.to_account_info(),
        a.user_usdc_ta.to_account_info(),
        a.usdc_vault_auth.to_account_info(),
        usdc_out.bits,
        a.usdc_mint.decimals,
        usdc_vault_seeds,
    )?;

    let lst_after = lst_before
        .checked_add(&requested)
        .ok_or(CoreError::DestinationCollateral)?;
    hylo.refresh_lst_vault(&lst_header.price_sol, lst_before, lst_after, epoch)?;
    usdc_pair.virtual_stablecoin.burn(usdc_out)?;

    let pool_balance = a.stablecoin_pool.amount;
    let mode = exchange.rebalance_mode();
    let stablecoin_auth_bump = hylo.stablecoin_auth_bump;
    let (burned, minted) = if matches!(pnl, RebalancePnl::Loss(_)) {
        drop(hylo);
        drop(usdc_pair);
        drop(lst_header);
        let cpi = pnl_cpi_from_lst(&a, stablecoin_auth_bump)?;
        let out = settle_pnl(
            pnl,
            &mut VirtualStablecoin::new(),
            &mut PoolDrawdown::default(),
            SUPPLY_FLOOR,
            mode,
            pool_balance,
            cpi,
        )?;
        let mut hylo = a.hylo.load_mut()?;
        let hylo = &mut *hylo;
        apply_rebalance_loss(
            &mut hylo.virtual_stablecoin,
            &mut hylo.pool_drawdown,
            out.0,
            SUPPLY_FLOOR,
        )?;
        out
    } else {
        let cpi = pnl_cpi_from_lst(&a, stablecoin_auth_bump)?;
        let hylo = &mut *hylo;
        settle_pnl(
            pnl,
            &mut hylo.virtual_stablecoin,
            &mut hylo.pool_drawdown,
            SUPPLY_FLOOR,
            mode,
            pool_balance,
            cpi,
        )?
    };

    Ok((
        SwapLstToUsdcEvent {
            lst_mint: a.lst_mint.key(),
            lst_deposited: requested.into(),
            sol_rebalance_usd_price: lst_sol_price.into(),
            usdc_withdrawn: usdc_out.into(),
            usdc_usd_price: oracle_event(usdc_oracle),
        },
        SettleRebalancePnlLstEvent {
            pnl: pnl.into(),
            stablecoin_burned: burned.into(),
            stablecoin_minted: minted.into(),
        },
    ))
}

#[inline(never)]
pub fn swap_usdc_to_lst(
    a: LstUsdcAccounts,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<(SwapUsdcToLstEvent, SettleRebalancePnlLstEvent)> {
    require!(amount > 0, CoreError::ZeroAmount);
    let mut hylo = a.hylo.load_mut()?;
    let mut usdc_pair = a.usdc_pair.load_mut()?;
    let lst_header = a.lst_header.load()?;
    let clock = Clock::get()?;
    lst_gates(&hylo, &usdc_pair, clock.epoch())?;
    let usdc_oracle = assert_usdc_par(&clock, &usdc_pair, a.usdc_usd_pyth_feed)?;

    let epoch = clock.epoch();
    let sol_price_update =
        load_price_update(a.sol_usd_pyth_feed, &hylo_core::pyth::SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &hylo.total_sol_cache,
        hylo.stablecoin_mint_threshold()?,
        hylo.oracle_config()?,
        hylo.levercoin_fees,
        &sol_price_update,
        hylo.virtual_stablecoin,
        None,
        hylo.lst_sell_curve_config,
        hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.rebalance_sell_active(),
        ErrorCode::RebalanceSellInactive
    );

    let usdc_in = UFix64::<N6>::new(amount);
    require!(
        a.user_usdc_ta.amount >= usdc_in.bits,
        CoreError::InsufficientLiquidity
    );
    let stake_pool = SplStakePool::from_bytes(&a.pool_state.try_borrow_data()?)?;
    let max_usdc = exchange.max_rebalance_sell_usdc(
        stake_pool,
        lst_header.rebalance_fee()?,
        UFix64::new(a.lst_vault.amount),
        SUPPLY_FLOOR,
    )?;
    require!(usdc_in <= max_usdc, CoreError::InsufficientLiquidity);

    let adjusted = stake_pool
        .true_price()?
        .adjust_price(lst_header.rebalance_fee()?)?;
    let conversion = exchange.rebalance_sell_conversion(&adjusted, usdc_in)?;
    let lst_out = conversion.token_to_lst(usdc_in, UFix64::one())?;
    require!(lst_out > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        lst_out.bits <= a.lst_vault.amount,
        CoreError::InsufficientLiquidity
    );
    if let Some(slippage_config) = slippage_config.as_ref() {
        slippage_config.validate_token_out(lst_out)?;
    }

    let pnl = exchange.rebalance_pnl_sell_side(&lst_header.price_sol, lst_out, usdc_in)?;
    let lst_sol_price = conversion.lst_sol_price;

    let lst_mint_key = a.lst_mint.key();
    let lst_vault_bump = [a.lst_vault_auth_bump];
    let lst_vault_seeds: &[&[u8]] = &[VAULT_AUTH, lst_mint_key.as_ref(), &lst_vault_bump];

    let lst_before = UFix64::<N9>::new(a.lst_vault.amount);
    transfer_checked_user(
        a.token_program.to_account_info(),
        a.user_usdc_ta.to_account_info(),
        a.usdc_mint.to_account_info(),
        a.usdc_vault.to_account_info(),
        a.user.to_account_info(),
        usdc_in.bits,
        a.usdc_mint.decimals,
    )?;
    transfer_checked_pda(
        a.token_program.to_account_info(),
        a.lst_vault.to_account_info(),
        a.lst_mint.to_account_info(),
        a.user_lst_ta.to_account_info(),
        a.lst_vault_auth.to_account_info(),
        lst_out.bits,
        a.lst_mint.decimals,
        lst_vault_seeds,
    )?;

    let lst_after = lst_before
        .checked_sub(&lst_out)
        .ok_or(CoreError::DestinationCollateral)?;
    hylo.refresh_lst_vault(&lst_header.price_sol, lst_before, lst_after, epoch)?;
    usdc_pair.virtual_stablecoin.mint(usdc_in)?;

    let pool_balance = a.stablecoin_pool.amount;
    let mode = exchange.rebalance_mode();
    let stablecoin_auth_bump = hylo.stablecoin_auth_bump;
    let (burned, minted) = if matches!(pnl, RebalancePnl::Loss(_)) {
        drop(hylo);
        drop(usdc_pair);
        drop(lst_header);
        let cpi = pnl_cpi_from_lst(&a, stablecoin_auth_bump)?;
        let out = settle_pnl(
            pnl,
            &mut VirtualStablecoin::new(),
            &mut PoolDrawdown::default(),
            SUPPLY_FLOOR,
            mode,
            pool_balance,
            cpi,
        )?;
        let mut hylo = a.hylo.load_mut()?;
        let hylo = &mut *hylo;
        apply_rebalance_loss(
            &mut hylo.virtual_stablecoin,
            &mut hylo.pool_drawdown,
            out.0,
            SUPPLY_FLOOR,
        )?;
        out
    } else {
        let cpi = pnl_cpi_from_lst(&a, stablecoin_auth_bump)?;
        let hylo = &mut *hylo;
        settle_pnl(
            pnl,
            &mut hylo.virtual_stablecoin,
            &mut hylo.pool_drawdown,
            SUPPLY_FLOOR,
            mode,
            pool_balance,
            cpi,
        )?
    };

    Ok((
        SwapUsdcToLstEvent {
            lst_mint: a.lst_mint.key(),
            usdc_deposited: usdc_amount_n9(usdc_in)?.into(),
            usdc_usd_price: oracle_event(usdc_oracle),
            lst_withdrawn: lst_out.into(),
            sol_rebalance_usd_price: lst_sol_price.into(),
        },
        SettleRebalancePnlLstEvent {
            pnl: pnl.into(),
            stablecoin_burned: burned.into(),
            stablecoin_minted: minted.into(),
        },
    ))
}

fn pnl_cpi_from_exo<'info>(
    a: &ExoUsdcAccounts<'_, 'info>,
    stablecoin_auth_bump: u8,
) -> Result<PnlCpi<'info>> {
    Ok(PnlCpi {
        hylo: a.hylo.to_account_info(),
        pool_config: a.pool_config.to_account_info(),
        pool_auth: a.pool_auth.to_account_info(),
        settlement_auth: a.settlement_auth.to_account_info(),
        stablecoin_pool: a.stablecoin_pool.to_account_info(),
        stablecoin_mint: a.stablecoin_mint.to_account_info(),
        stablecoin_mint_auth: a.stablecoin_mint_auth.to_account_info(),
        token_program: a.token_program.to_account_info(),
        earn_pool: a.earn_pool.to_account_info(),
        stablecoin_mint_key: a.stablecoin_mint.key(),
        stablecoin_auth_bump,
        settlement_auth_bump: a.settlement_auth_bump,
    })
}

fn load_exo_context<'info>(
    clock: Clock,
    a: &ExoUsdcAccounts<'_, 'info>,
    exo_pair: &ExoPair,
    price_update: &PriceUpdateV2,
) -> Result<ExoExchangeContext<Clock>> {
    let total_collateral = normalize_mint_exp(a.collateral_mint, a.collateral_vault.amount)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    Ok(ExoExchangeContext::load(
        clock,
        total_collateral,
        exo_pair.stablecoin_mint_threshold()?,
        exo_pair.oracle_config()?,
        exo_pair.levercoin_fees,
        price_update,
        exo_pair.virtual_stablecoin,
        Some(a.levercoin_mint),
        exo_pair.sell_curve_config,
        exo_pair.buy_curve_config,
        exo_pair.levercoin_market_cap_limit.try_into()?,
    )?)
}

fn assert_exo_oracle(a: &ExoUsdcAccounts, exo_pair: &ExoPair) -> Result<PriceUpdateV2> {
    require_keys_eq!(
        a.collateral_usd_pyth_feed.key(),
        exo_pair.oracle,
        ErrorCode::ExoOracleInvalid
    );
    let mut oracle_data: &[u8] = &a.collateral_usd_pyth_feed.try_borrow_data()?;
    let price_update = PriceUpdateV2::try_deserialize(&mut oracle_data)
        .map_err(|_| error!(ErrorCode::ExoOracleInvalid))?;
    require!(
        price_update.price_message.feed_id == exo_pair.oracle_feed_id,
        ErrorCode::ExoOracleInvalid
    );
    Ok(price_update)
}

/// `amount == None` uses the user's full collateral balance, capped by buy-side capacity.
#[inline(never)]
pub fn swap_exo_to_usdc(
    a: ExoUsdcAccounts,
    amount: Option<u64>,
    slippage_config: Option<SlippageConfig>,
) -> Result<(SwapExoToUsdcEvent, SettleRebalancePnlExoEvent)> {
    let hylo = a.hylo.load()?;
    let mut exo_pair = a.exo_pair.load_mut()?;
    let mut usdc_pair = a.usdc_pair.load_mut()?;
    let clock = Clock::get()?;
    exo_gates(&hylo, &exo_pair, &usdc_pair, clock.epoch())?;
    let usdc_oracle = assert_usdc_par(&clock, &usdc_pair, a.usdc_usd_pyth_feed)?;
    let collateral_price_update = assert_exo_oracle(&a, &exo_pair)?;
    let exchange = load_exo_context(clock, &a, &exo_pair, &collateral_price_update)?;
    require!(
        exchange.rebalance_buy_active(),
        ErrorCode::RebalanceBuyInactive
    );

    let buy_target = exchange.rebalance_buy_target()?;
    let buy_target_native = denormalize_mint_exp(a.collateral_mint, buy_target)?;
    let requested_native = match amount {
        Some(raw) => {
            require!(raw > 0, CoreError::ZeroAmount);
            let requested = normalize_mint_exp(a.collateral_mint, raw)
                .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
            require!(
                requested <= buy_target,
                CoreError::RebalanceBuyTargetExceeded
            );
            raw
        }
        None => {
            require!(a.user_collateral_ta.amount > 0, CoreError::ZeroAmount);
            a.user_collateral_ta.amount.min(buy_target_native)
        }
    };
    require!(requested_native > 0, CoreError::ZeroAmount);
    require!(
        a.user_collateral_ta.amount >= requested_native,
        CoreError::InsufficientLiquidity
    );
    let requested = normalize_mint_exp(a.collateral_mint, requested_native)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    require!(requested > UFix64::zero(), CoreError::ZeroAmount);

    let conversion = exchange.rebalance_buy_conversion(requested)?;
    let mut requested = requested;
    let mut requested_native = requested_native;
    let mut usdc_out = conversion.exo_to_token(requested, UFix64::one())?;
    if amount.is_none() && usdc_out.bits > a.usdc_collateral_vault.amount {
        requested =
            conversion.token_to_exo(UFix64::new(a.usdc_collateral_vault.amount), UFix64::one())?;
        require!(requested > UFix64::zero(), CoreError::ZeroAmount);
        requested_native = denormalize_mint_exp(a.collateral_mint, requested)?;
        require!(requested_native > 0, CoreError::ZeroAmount);
        let conversion = exchange.rebalance_buy_conversion(requested)?;
        usdc_out = conversion.exo_to_token(requested, UFix64::one())?;
    }
    require!(usdc_out > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        usdc_out.bits <= a.usdc_collateral_vault.amount,
        CoreError::InsufficientLiquidity
    );
    usdc_pair
        .virtual_stablecoin
        .supply()?
        .checked_sub(&usdc_out)
        .ok_or(CoreError::BurnUnderflow)?;
    if let Some(slippage_config) = slippage_config.as_ref() {
        slippage_config.validate_token_out(usdc_out)?;
    }

    let pnl = exchange.rebalance_pnl_buy_side(requested, usdc_out)?;
    let projected = exchange.projected_rebalance_buy_state(requested)?;
    let curve_price = exchange
        .rebalance_buy_curve()?
        .price(projected.collateral_ratio)?;
    let floor: UFix64<N6> = exo_pair.virtual_stablecoin_supply_floor.try_into()?;

    let usdc_mint_key = a.usdc_mint.key();
    let usdc_vault_bump = [usdc_pair.vault_auth_bump];
    let usdc_vault_seeds: &[&[u8]] = &[USDC_VAULT_AUTH, usdc_mint_key.as_ref(), &usdc_vault_bump];

    transfer_checked_user(
        a.token_program.to_account_info(),
        a.user_collateral_ta.to_account_info(),
        a.collateral_mint.to_account_info(),
        a.collateral_vault.to_account_info(),
        a.user.to_account_info(),
        requested_native,
        a.collateral_mint.decimals,
    )?;
    transfer_checked_pda(
        a.token_program.to_account_info(),
        a.usdc_collateral_vault.to_account_info(),
        a.usdc_mint.to_account_info(),
        a.user_usdc_ta.to_account_info(),
        a.usdc_vault_auth.to_account_info(),
        usdc_out.bits,
        a.usdc_mint.decimals,
        usdc_vault_seeds,
    )?;
    usdc_pair.virtual_stablecoin.burn(usdc_out)?;

    let pool_balance = a.stablecoin_pool.amount;
    let stablecoin_auth_bump = hylo.stablecoin_auth_bump;
    drop(hylo);
    let cpi = pnl_cpi_from_exo(&a, stablecoin_auth_bump)?;
    let exo_pair = &mut *exo_pair;
    let (burned, minted) = settle_pnl(
        pnl,
        &mut exo_pair.virtual_stablecoin,
        &mut exo_pair.pool_drawdown,
        floor,
        exchange.rebalance_mode(),
        pool_balance,
        cpi,
    )?;
    apply_rebalance_loss(
        &mut exo_pair.virtual_stablecoin,
        &mut exo_pair.pool_drawdown,
        burned,
        floor,
    )?;

    Ok((
        SwapExoToUsdcEvent {
            collateral_mint: a.collateral_mint.key(),
            collateral_deposited: requested.into(),
            collateral_usd_price: curve_price.into(),
            usdc_withdrawn: usdc_out.into(),
            usdc_usd_price: oracle_event(usdc_oracle),
        },
        SettleRebalancePnlExoEvent {
            collateral_mint: a.collateral_mint.key(),
            pnl: pnl.into(),
            stablecoin_burned: burned.into(),
            stablecoin_minted: minted.into(),
        },
    ))
}

#[inline(never)]
pub fn swap_usdc_to_exo(
    a: ExoUsdcAccounts,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<(SwapUsdcToExoEvent, SettleRebalancePnlExoEvent)> {
    require!(amount > 0, CoreError::ZeroAmount);
    let hylo = a.hylo.load()?;
    let mut exo_pair = a.exo_pair.load_mut()?;
    let mut usdc_pair = a.usdc_pair.load_mut()?;
    let clock = Clock::get()?;
    exo_gates(&hylo, &exo_pair, &usdc_pair, clock.epoch())?;
    let usdc_oracle = assert_usdc_par(&clock, &usdc_pair, a.usdc_usd_pyth_feed)?;
    let collateral_price_update = assert_exo_oracle(&a, &exo_pair)?;
    let exchange = load_exo_context(clock, &a, &exo_pair, &collateral_price_update)?;
    require!(
        exchange.rebalance_sell_active(),
        ErrorCode::RebalanceSellInactive
    );

    let usdc_in = UFix64::<N6>::new(amount);
    require!(
        a.user_usdc_ta.amount >= usdc_in.bits,
        CoreError::InsufficientLiquidity
    );
    let floor: UFix64<N6> = exo_pair.virtual_stablecoin_supply_floor.try_into()?;
    let max_usdc = exchange.max_rebalance_sell_usdc(floor)?;
    require!(usdc_in <= max_usdc, CoreError::InsufficientLiquidity);

    let conversion = exchange.rebalance_sell_conversion(usdc_in)?;
    let collateral_out_n9 = conversion.token_to_exo(usdc_in, UFix64::one())?;
    require!(collateral_out_n9 > UFix64::zero(), CoreError::ZeroAmount);
    let collateral_out = denormalize_mint_exp(a.collateral_mint, collateral_out_n9)?;
    require!(collateral_out > 0, CoreError::ZeroAmount);
    require!(
        collateral_out <= a.collateral_vault.amount,
        CoreError::InsufficientLiquidity
    );
    if let Some(slippage_config) = slippage_config.as_ref() {
        slippage_config.validate_token_out_normalized(a.collateral_mint, collateral_out_n9)?;
    }

    let pnl = exchange.rebalance_pnl_sell_side(collateral_out_n9, usdc_in)?;
    let projected = exchange.projected_rebalance_sell_state(usdc_in)?;
    let curve_price = exchange
        .rebalance_sell_curve()?
        .price(projected.collateral_ratio)?;

    let collateral_mint_key = a.collateral_mint.key();
    let vault_bump = [exo_pair.vault_auth_bump];
    let vault_seeds: &[&[u8]] = &[EXO_VAULT_AUTH, collateral_mint_key.as_ref(), &vault_bump];

    transfer_checked_user(
        a.token_program.to_account_info(),
        a.user_usdc_ta.to_account_info(),
        a.usdc_mint.to_account_info(),
        a.usdc_collateral_vault.to_account_info(),
        a.user.to_account_info(),
        usdc_in.bits,
        a.usdc_mint.decimals,
    )?;
    transfer_checked_pda(
        a.token_program.to_account_info(),
        a.collateral_vault.to_account_info(),
        a.collateral_mint.to_account_info(),
        a.user_collateral_ta.to_account_info(),
        a.vault_auth.to_account_info(),
        collateral_out,
        a.collateral_mint.decimals,
        vault_seeds,
    )?;
    usdc_pair.virtual_stablecoin.mint(usdc_in)?;

    let pool_balance = a.stablecoin_pool.amount;
    let stablecoin_auth_bump = hylo.stablecoin_auth_bump;
    drop(hylo);
    let cpi = pnl_cpi_from_exo(&a, stablecoin_auth_bump)?;
    let exo_pair = &mut *exo_pair;
    let (burned, minted) = settle_pnl(
        pnl,
        &mut exo_pair.virtual_stablecoin,
        &mut exo_pair.pool_drawdown,
        floor,
        exchange.rebalance_mode(),
        pool_balance,
        cpi,
    )?;
    apply_rebalance_loss(
        &mut exo_pair.virtual_stablecoin,
        &mut exo_pair.pool_drawdown,
        burned,
        floor,
    )?;

    Ok((
        SwapUsdcToExoEvent {
            collateral_mint: a.collateral_mint.key(),
            usdc_deposited: usdc_amount_n9(usdc_in)?.into(),
            usdc_usd_price: oracle_event(usdc_oracle),
            collateral_withdrawn: collateral_out_n9.into(),
            collateral_usd_price: curve_price.into(),
        },
        SettleRebalancePnlExoEvent {
            collateral_mint: a.collateral_mint.key(),
            pnl: pnl.into(),
            stablecoin_burned: burned.into(),
            stablecoin_minted: minted.into(),
        },
    ))
}
