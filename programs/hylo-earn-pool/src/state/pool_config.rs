use anchor_lang::prelude::*;

use super::*;

#[account]
pub struct PoolConfig {
    pub _dead_admin: Pubkey,
    pub pool_auth_bump: u8,
    pub lp_token_auth_bump: u8,
    pub lp_token_mint_bump: u8,
    pub withdrawal_fee: UFixValue64,
    pub paused: bool,
    pub withdrawal_limiter: WithdrawalLimiter,
    pub deposit_limiter: DepositLimiter,
    pub _reserved: [u8; 19],
}
