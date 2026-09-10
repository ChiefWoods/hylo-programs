use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedAdd, CheckedSub, MulDiv, UFix64, N6, N9};
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::pyth::SOL_USD;
use hylo_core::rebalance::mode::RebalanceMode;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::stablecoin_ops::{drawdown_repay, mint_stablecoin};
use crate::lst_registry;
use crate::oracle::{load_price_update, oracle_event};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct HarvestYield<'info> {
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = lst_registry,
        has_one = stablecoin_mint,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
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
    pub stablecoin_fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = stablecoin_fee_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub hylo_earn_pool: UncheckedAccount<'info>,
    /// CHECK: Validated owner.
    #[account(
        owner = solana_sdk_ids::address_lookup_table::ID
    )]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<HarvestYield>) -> Result<HarvestYieldEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let clock = Clock::get()?;
    let epoch = clock.epoch;
    require!(
        hylo.yield_harvest_cache.is_stale(epoch),
        ErrorCode::YieldHarvestAlreadyRun
    );
    hylo.total_sol_cache.get_validated(epoch)?;

    lst_registry::remaining_matches_table(
        &ctx.accounts.lst_registry.try_borrow_data()?,
        ctx.remaining_accounts,
    )?;

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
    let rebalance_mode = exchange.rebalance_mode();
    let sol_usd = exchange.collateral_oracle_price();

    let blocks = &ctx.remaining_accounts[LST_REGISTRY_CALCULATOR_PREAMBLE_LEN..];
    let mut total_sol_harvested = UFix64::<N9>::zero();
    let mut skip_yield_harvest = rebalance_mode < RebalanceMode::Neutral;

    for block in blocks.chunks_exact(LST_REGISTRY_BLOCK_LEN) {
        let header_info = &block[0];
        require!(header_info.is_writable, ErrorCode::LstBlockInvalid);
        let mut header = lst_registry::load_header(header_info)?;
        let mint_info = &block[1];
        let vault = lst_registry::load_vault(&block[2])?;
        let pool_state_info = &block[3];

        require_keys_eq!(header.mint, *mint_info.key, ErrorCode::LstBlockInvalid);
        require_keys_eq!(header.vault, *block[2].key, ErrorCode::LstBlockInvalid);
        require_keys_eq!(
            header.pool_state,
            *pool_state_info.key,
            ErrorCode::LstBlockInvalid
        );
        require!(header.price_sol.epoch == epoch, ErrorCode::LstPriceOutdated);

        if header.prev_price_sol.epoch < header.price_sol.epoch {
            let current_price: UFix64<N9> = header.price_sol.price.try_into()?;
            let previous_price: UFix64<N9> = header.prev_price_sol.price.try_into()?;
            skip_yield_harvest |= current_price < previous_price;
        }

        if !skip_yield_harvest && header.prev_price_sol.epoch < header.price_sol.epoch {
            let delta = header
                .price_sol
                .checked_delta(&header.prev_price_sol)
                .map_err(|_| error!(ErrorCode::LstPriceDelta))?;
            let vault_amount = UFix64::<N9>::new(vault.amount);
            let sol_delta = delta
                .mul_div_floor(vault_amount, UFix64::one())
                .ok_or_else(|| error!(ErrorCode::LstSolAppreciation))?;
            total_sol_harvested = total_sol_harvested
                .checked_add(&sol_delta)
                .ok_or_else(|| error!(ErrorCode::LstAdditionOverflow))?;
        }

        header.last_yield_harvest_epoch = epoch;
        lst_registry::save_header(header_info, &header)?;
    }

    if skip_yield_harvest {
        let pool_balance = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
        hylo.yield_harvest_cache
            .update(pool_balance, UFix64::zero(), epoch)?;

        let event = HarvestYieldEvent {
            total_sol_harvested: UFix64::<N9>::zero().into(),
            fees_extracted: UFix64::<N6>::zero().into(),
            token_to_pool: UFix64::<N6>::zero().into(),
            pool_drawdown_repaid: UFix64::<N6>::zero().into(),
            sol_usd_price: oracle_event(sol_usd),
        };
        emit_cpi!(event.clone());
        return Ok(event);
    }

    let usd_yield_n9 = total_sol_harvested
        .mul_div_floor(sol_usd.spot, UFix64::one())
        .ok_or_else(|| error!(ErrorCode::LstSolAppreciation))?;
    let usd_yield: UFix64<N6> = usd_yield_n9
        .checked_convert()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))?;

    let allocated = hylo
        .yield_harvest_config
        .apply_allocation(usd_yield)
        .map_err(|_| error!(ErrorCode::YieldHarvestAllocation))?;
    let extract = hylo
        .yield_harvest_config
        .apply_fee(allocated)
        .map_err(|_| error!(ErrorCode::YieldHarvestAllocation))?;

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
        hylo.virtual_stablecoin.mint(minted)?;
    }
    let pool_drawdown_repaid = drawdown_repay(&mut hylo.pool_drawdown, extract.amount_remaining)?;
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
    hylo.yield_harvest_cache
        .update(pool_balance, net_to_pool, epoch)?;

    let event = HarvestYieldEvent {
        total_sol_harvested: total_sol_harvested.into(),
        fees_extracted: extract.fees_extracted.into(),
        token_to_pool: extract.amount_remaining.into(),
        pool_drawdown_repaid: pool_drawdown_repaid.into(),
        sol_usd_price: oracle_event(sol_usd),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
