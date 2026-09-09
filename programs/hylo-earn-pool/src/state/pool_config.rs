use anchor_lang::prelude::*;
use fix::prelude::{UFix64, UFixValue64, N4};

use super::*;

#[account(zero_copy(unsafe))]
#[repr(C)]
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

impl PoolConfig {
    pub fn update_withdrawal_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        Self::validate_withdrawal_fee(new_fee)?;
        require!(
            self.withdrawal_fee != new_fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.withdrawal_fee = new_fee;
        Ok(())
    }

    pub fn update_withdrawal_limit(
        &mut self,
        new_limit: UFixValue64,
        current_epoch: u64,
    ) -> Result<()> {
        require!(
            self.withdrawal_limiter.limit != new_limit,
            crate::error::ErrorCode::AdminNoop
        );
        self.withdrawal_limiter
            .update_limit(new_limit, current_epoch)?;
        Ok(())
    }

    pub fn update_deposit_limit(
        &mut self,
        pool_amount: UFix64<fix::prelude::N6>,
        new_limit: UFixValue64,
    ) -> Result<()> {
        require!(
            self.deposit_limiter.limit != new_limit,
            crate::error::ErrorCode::AdminNoop
        );
        self.deposit_limiter.update_limit(pool_amount, new_limit)?;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        require!(!self.paused, crate::error::ErrorCode::AdminNoop);
        self.paused = true;
        Ok(())
    }

    pub fn unpause(&mut self) -> Result<()> {
        require!(self.paused, crate::error::ErrorCode::AdminNoop);
        self.paused = false;
        Ok(())
    }

    fn validate_withdrawal_fee(fee: UFixValue64) -> Result<()> {
        let fee: UFix64<N4> = fee.try_into()?;
        require!(
            fee <= UFix64::constant(1_000),
            hylo_core::error::CoreError::InvalidFees
        );
        Ok(())
    }
}
