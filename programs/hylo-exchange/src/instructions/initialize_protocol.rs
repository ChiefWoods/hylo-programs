use anchor_lang::prelude::*;
use hylo_core::{
    fees::controller::FeeController,
    pyth::{validate_interval_secs, SOL_USD},
    rebalance::{mode::validate_stablecoin_mint_threshold, pricing::RebalanceCurveConfig},
};

use crate::constants::*;
use crate::error::ErrorCode;
use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub upgrade_authority: Signer<'info>,
    #[account(init, payer = admin, space = Hylo::DISCRIMINATOR.len() + core::mem::size_of::<Hylo>(), seeds = [HYLO], bump)]
    pub hylo: AccountLoader<'info, Hylo>,
    /// CHECK: IDL metadata: no additional constraints.
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = hylo_exchange.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_exchange: Program<'info, HyloExchange>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()) @ ErrorCode::AddressChangeUpgradeAuthority
    )]
    pub program_data: Account<'info, ProgramData>,
}

pub fn handler(
    ctx: Context<InitializeProtocol>,
    pause_authority: Pubkey,
    oracle_interval_secs: u64,
    stablecoin_mint_threshold: UFixValue64,
    levercoin_fees: LevercoinFees,
    yield_harvest_config: YieldHarvestConfig,
) -> Result<()> {
    let oracle_interval_secs = validate_interval_secs(oracle_interval_secs)?;
    let stablecoin_mint_threshold = validate_stablecoin_mint_threshold(stablecoin_mint_threshold)?;
    let levercoin_fees = levercoin_fees.validate()?;
    let yield_harvest_config = yield_harvest_config.validate()?;
    let current_epoch = Clock::get()?.epoch;

    let zero_fee = UFixValue64::new(0, -4);
    let zero_pct = UFixValue64::new(0, -9);
    let mut yield_harvest_cache = HarvestCache {
        epoch: 0,
        stability_pool_cap: UFixValue64::new(0, -6),
        stablecoin_to_pool: UFixValue64::new(0, -6),
    };
    yield_harvest_cache.init(current_epoch)?;

    *ctx.accounts.hylo.load_init()? = Hylo {
        admin: ctx.accounts.admin.key(),
        treasury: ctx.accounts.treasury.key(),
        lst_registry: Pubkey::default(),
        stablecoin_mint: Pubkey::default(),
        levercoin_mint: Pubkey::default(),
        pause_authority,
        stablecoin_mint_bump: 0,
        stablecoin_auth_bump: 0,
        levercoin_mint_bump: 0,
        levercoin_auth_bump: 0,
        registry_auth_bump: 0,
        total_sol_cache_bump: 0,
        oracle_interval_secs,
        stablecoin_fees: StablecoinFees::new(
            FeePair::new(zero_fee, zero_fee),
            FeePair::new(zero_fee, zero_fee),
        ),
        levercoin_fees,
        total_sol_cache: TotalSolCache::new(current_epoch),
        yield_harvest_cache,
        yield_harvest_config,
        stablecoin_mint_threshold,
        _unused_1: zero_pct,
        oracle_conf_tolerance: DEFAULT_ORACLE_CONF_TOLERANCE,
        sol_usd_oracle: SOL_USD.address,
        lst_swap_fee: zero_fee,
        virtual_stablecoin: VirtualStablecoin::new(),
        lst_buy_curve_config: RebalanceCurveConfig::new(zero_pct, zero_pct),
        lst_sell_curve_config: RebalanceCurveConfig::new(zero_pct, zero_pct),
        protocol_paused: false,
        lst_pair_paused: false,
        _unused_2: zero_pct,
        pool_drawdown: PoolDrawdown::default(),
        _reserved: [0; 13],
    };

    Ok(())
}
