use anchor_lang::prelude::*;

use super::*;

/// Header for a specific LST. Should be the first entry in the block of
/// accounts, and contains references to the rest of the accounts.
#[account]
pub struct LstHeader {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub pool_state: Pubkey,
    pub stake_program: LstStakePoolProgram,
    pub prev_price_sol: LstSolPrice,
    pub price_sol: LstSolPrice,
    pub last_yield_harvest_epoch: u64,
    pub rebalance_fee: UFixValue64,
    pub _reserved: [u8; 55],
}
