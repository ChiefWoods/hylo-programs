use anchor_lang::prelude::*;

use super::*;

#[account]
pub struct UsdcPair {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub swap_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub paused: bool,
    pub _reserved: [u8; 127],
}
