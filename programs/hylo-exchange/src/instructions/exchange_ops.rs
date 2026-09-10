use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use fix::prelude::{UFix64, N6, N9};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::ExoExchangeContext;
use hylo_core::solana_clock::SolanaClock;
use hylo_core::util::{denormalize_mint_exp, normalize_mint_exp};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::error::ErrorCode;
use crate::oracle::load_price_update;
use crate::state::{ExoPair, Hylo};

pub(crate) fn lst_user_gates(hylo: &Hylo, epoch: u64) -> Result<()> {
    require!(!hylo.protocol_paused, CoreError::ProtocolPaused);
    require!(!hylo.lst_pair_paused, CoreError::PairPaused);
    require!(
        hylo.yield_harvest_cache.epoch == epoch,
        CoreError::YieldHarvestNotRun
    );
    Ok(())
}

pub(crate) fn require_exo_genesis(exo_pair: &ExoPair) -> Result<()> {
    let floor: UFix64<N6> = exo_pair.virtual_stablecoin_supply_floor.try_into()?;
    require!(
        floor > UFix64::zero(),
        ErrorCode::ExoPairZeroVirtualStablecoin
    );
    Ok(())
}

pub(crate) fn exo_user_gates(hylo: &Hylo, exo_pair: &ExoPair, epoch: u64) -> Result<()> {
    require!(!hylo.protocol_paused, CoreError::ProtocolPaused);
    require!(!exo_pair.paused, CoreError::PairPaused);
    require_exo_genesis(exo_pair)?;
    require!(
        exo_pair.borrow_rate_harvest_cache.epoch == epoch,
        CoreError::BorrowRateHarvestNotRun
    );
    Ok(())
}

pub(crate) fn load_exo_price_update(
    feed: &UncheckedAccount,
    exo_pair: &ExoPair,
) -> Result<PriceUpdateV2> {
    require_keys_eq!(feed.key(), exo_pair.oracle, ErrorCode::ExoOracleInvalid);
    load_price_update(feed, &exo_pair.oracle_feed_id)
        .map_err(|_| error!(ErrorCode::ExoOracleInvalid))
}

pub(crate) fn load_exo_exchange<C: SolanaClock>(
    clock: C,
    exo_pair: &ExoPair,
    collateral_mint: &Mint,
    collateral_vault: &TokenAccount,
    price_update: &PriceUpdateV2,
    levercoin_mint: Option<&Mint>,
) -> Result<ExoExchangeContext<C>> {
    let total_collateral = normalize_mint_exp(collateral_mint, collateral_vault.amount)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    Ok(ExoExchangeContext::load(
        clock,
        total_collateral,
        exo_pair.stablecoin_mint_threshold()?,
        exo_pair.oracle_config()?,
        exo_pair.levercoin_fees,
        price_update,
        exo_pair.virtual_stablecoin,
        levercoin_mint,
        exo_pair.sell_curve_config,
        exo_pair.buy_curve_config,
        exo_pair.levercoin_market_cap_limit.try_into()?,
    )?)
}

pub(crate) fn split_collateral_native(
    mint: &Mint,
    amount: u64,
    fees_n9: UFix64<N9>,
) -> Result<(u64, u64, UFix64<N9>)> {
    let fee_native = denormalize_mint_exp(mint, fees_n9)?;
    let net_native = amount
        .checked_sub(fee_native)
        .ok_or(CoreError::FeeExtraction)?;
    let net_n9 = normalize_mint_exp(mint, net_native)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    Ok((fee_native, net_native, net_n9))
}

pub(crate) fn split_gross_native(
    mint: &Mint,
    gross_n9: UFix64<N9>,
    fees_n9: UFix64<N9>,
) -> Result<(u64, u64)> {
    let fee_native = denormalize_mint_exp(mint, fees_n9)?;
    let gross_native = denormalize_mint_exp(mint, gross_n9)?;
    let net_native = gross_native
        .checked_sub(fee_native)
        .ok_or(CoreError::FeeExtraction)?;
    Ok((fee_native, net_native))
}
