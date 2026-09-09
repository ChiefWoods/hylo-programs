use anchor_lang::prelude::*;

use super::*;

#[account]
#[derive(InitSpace)]
pub struct Hylo {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub lst_registry: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub pause_authority: Pubkey,
    pub stablecoin_mint_bump: u8,
    pub stablecoin_auth_bump: u8,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub registry_auth_bump: u8,
    pub total_sol_cache_bump: u8,
    pub oracle_interval_secs: u64,
    pub stablecoin_fees: StablecoinFees,
    pub levercoin_fees: LevercoinFees,
    pub total_sol_cache: TotalSolCache,
    pub yield_harvest_cache: HarvestCache,
    pub yield_harvest_config: YieldHarvestConfig,
    pub stablecoin_mint_threshold: UFixValue64,
    pub _unused_1: UFixValue64,
    pub oracle_conf_tolerance: UFixValue64,
    pub sol_usd_oracle: Pubkey,
    pub lst_swap_fee: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub lst_buy_curve_config: RebalanceCurveConfig,
    pub lst_sell_curve_config: RebalanceCurveConfig,
    pub protocol_paused: bool,
    pub lst_pair_paused: bool,
    pub _unused_2: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub _reserved: [u8; 13],
}
