use anchor_lang::prelude::*;
use fix::prelude::{UFix64, UFixValue64, N5, N9};
use std::cmp::Ordering;

use super::*;

/// Header for a specific LST. Should be the first entry in the block of
/// accounts, and contains references to the rest of the accounts.
#[account]
#[derive(InitSpace)]
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

impl LstHeader {
    pub fn price_sol(&self, current_epoch: u64) -> Result<UFix64<N9>> {
        Ok(self.price_sol.get_epoch_price(current_epoch)?)
    }

    pub fn rebalance_fee(&self) -> Result<UFix64<N5>> {
        Ok(self.rebalance_fee.try_into()?)
    }

    pub fn update_price(&mut self, new_price: UFix64<N9>, current_epoch: u64) -> Result<()> {
        match current_epoch.cmp(&self.price_sol.epoch) {
            Ordering::Greater => {
                self.prev_price_sol = self.price_sol;
                self.price_sol = LstSolPrice::new(new_price.into(), current_epoch);
                Ok(())
            }
            Ordering::Equal => Ok(()),
            Ordering::Less => err!(crate::error::ErrorCode::LstPriceEpochsInvalid),
        }
    }

    pub fn update_rebalance_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        let fee: UFix64<N5> = new_fee.try_into()?;
        let maximum_fee = UFix64::constant(500);
        require!(fee <= maximum_fee, hylo_core::error::CoreError::InvalidFees);
        require!(
            self.rebalance_fee != new_fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.rebalance_fee = new_fee;
        Ok(())
    }
}
