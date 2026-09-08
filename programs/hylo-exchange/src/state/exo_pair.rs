use anchor_lang::prelude::*;

use super::*;

#[account]
pub struct ExoPair {
    pub collateral_mint: Pubkey,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub oracle: Pubkey,
    pub oracle_feed_id: [u8; 32],
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub stablecoin_mint_threshold: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub borrow_rate_config: BorrowRateConfig,
    pub borrow_rate_harvest_cache: HarvestCache,
    pub levercoin_fees: LevercoinFees,
    pub sell_curve_config: RebalanceCurveConfig,
    pub buy_curve_config: RebalanceCurveConfig,
    pub _unused_1: UFixValue64,
    pub paused: bool,
    pub levercoin_market_cap_limit: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub virtual_stablecoin_supply_floor: UFixValue64,
    pub _reserved: [u8; 91],
}
