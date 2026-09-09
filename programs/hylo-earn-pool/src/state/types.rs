use std::cmp::Ordering;

use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;
use fix::prelude::{CheckedAdd, CheckedSub, UFix64, N6};
use hylo_core::error::CoreError;
use hylo_core::error::CoreError::{
    WithdrawalLimitArithmetic, WithdrawalLimitExceededForEpoch, WithdrawalLimitInvalidEpoch,
    WithdrawalLimitValidation,
};
use hylo_core::virtual_stablecoin::VirtualStablecoin;

type CoreResult<T> = std::result::Result<T, CoreError>;

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}

/// Per-epoch withdrawal window, reset lazily on epoch rollover.
///
/// This remains local because `hylo_core::limiter::withdraw::WithdrawalLimiter::new`
/// is gated behind `offchain`, whose dependency graph cannot build for SBF. Keep this
/// implementation aligned with the core limiter while allowing onchain initialization.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub struct WithdrawalLimiter {
    pub limit: UFixValue64,
    withdrawal_ledger: VirtualStablecoin,
    epoch: u64,
}

impl WithdrawalLimiter {
    #[must_use]
    pub fn new(
        limit: UFixValue64,
        withdrawal_ledger: VirtualStablecoin,
        epoch: u64,
    ) -> WithdrawalLimiter {
        WithdrawalLimiter {
            limit,
            withdrawal_ledger,
            epoch,
        }
    }

    pub fn limit(&self) -> CoreResult<UFix64<N6>> {
        Ok(self.limit.try_into()?)
    }

    pub fn update_limit(
        &mut self,
        new_limit_raw: UFixValue64,
        current_epoch: u64,
    ) -> CoreResult<()> {
        let new_limit: UFix64<N6> = new_limit_raw.try_into()?;
        if current_epoch < self.epoch {
            Err(WithdrawalLimitInvalidEpoch)
        } else if new_limit > UFix64::zero() {
            self.limit = new_limit_raw;
            self.withdrawal_ledger = VirtualStablecoin::new();
            self.epoch = current_epoch;
            Ok(())
        } else {
            Err(WithdrawalLimitValidation)
        }
    }

    pub fn register_withdrawal(
        &mut self,
        withdrawal: UFix64<N6>,
        current_epoch: u64,
    ) -> CoreResult<()> {
        let validated = self.validate_withdrawal(withdrawal, current_epoch)?;
        let mut ledger = self.epoch_ledger(current_epoch)?;
        ledger.mint(validated)?;
        self.withdrawal_ledger = ledger;
        self.epoch = current_epoch;
        Ok(())
    }

    pub fn validate_withdrawal(
        &self,
        withdrawal: UFix64<N6>,
        current_epoch: u64,
    ) -> CoreResult<UFix64<N6>> {
        let ledger_total = self.epoch_ledger(current_epoch)?.supply()?;
        let projected = ledger_total
            .checked_add(&withdrawal)
            .ok_or(WithdrawalLimitArithmetic)?;
        if projected <= self.limit()? {
            Ok(withdrawal)
        } else {
            Err(WithdrawalLimitExceededForEpoch)
        }
    }

    pub fn max_withdrawal(&self, current_epoch: u64) -> CoreResult<UFix64<N6>> {
        let ledger_total = self.epoch_ledger(current_epoch)?.supply()?;
        let projected = self.limit()?.checked_sub(&ledger_total);
        Ok(projected.unwrap_or_default())
    }

    fn epoch_ledger(&self, current_epoch: u64) -> CoreResult<VirtualStablecoin> {
        match current_epoch.cmp(&self.epoch) {
            Ordering::Less => Err(WithdrawalLimitInvalidEpoch),
            Ordering::Equal => Ok(self.withdrawal_ledger),
            Ordering::Greater => Ok(VirtualStablecoin::new()),
        }
    }
}
