use crate::state::*;
use anchor_lang::prelude::*;

#[event]
pub struct AbsorbLossEvent {
    pub requested_loss: UFixValue64,
    pub amount_stablecoin_burned: UFixValue64,
    pub remaining_pool_balance: UFixValue64,
}

#[event]
pub struct PauseEvent {}

#[event]
pub struct UnpauseEvent {}

#[event]
pub struct UpdateDepositLimitEvent {
    pub old_deposit_limit: UFixValue64,
    pub new_deposit_limit: UFixValue64,
}

#[event]
pub struct UpdateWithdrawalFeeEvent {
    pub old_withdrawal_fee: UFixValue64,
    pub new_withdrawal_fee: UFixValue64,
}

#[event]
pub struct UpdateWithdrawalLimitEvent {
    pub old_withdrawal_limit: UFixValue64,
    pub new_withdrawal_limit: UFixValue64,
}

#[event]
pub struct UserDepositEvent {
    pub stablecoin_deposited: UFixValue64,
    pub lp_token_nav: UFixValue64,
    pub lp_token_minted: UFixValue64,
}

#[event]
pub struct UserWithdrawEvent {
    pub lp_token_burned: UFixValue64,
    pub stablecoin_withdrawn: UFixValue64,
    pub stablecoin_fees: UFixValue64,
}
