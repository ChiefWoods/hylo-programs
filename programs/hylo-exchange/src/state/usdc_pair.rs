use anchor_lang::prelude::*;

use super::*;

#[account]
#[derive(InitSpace)]
pub struct UsdcPair {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub mint_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub paused: bool,
    pub par_tolerance: ParTolerance,
    pub redeem_fee: UFixValue64,
    pub _reserved: [u8; 109],
}
