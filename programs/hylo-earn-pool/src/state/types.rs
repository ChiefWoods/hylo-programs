use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbsorbLossEvent {
    pub requested_loss: UFixValue64,
    pub amount_stablecoin_burned: UFixValue64,
    pub remaining_pool_balance: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct DepositLimiter {
    pub limit: UFixValue64,
}

/// Represents the spread of fees between mint and redeem for protocol tokens.
/// All fees must be in basis points to represent a fractional percentage
/// directly applicable to a token amount e.g. `0.XXXX` or `bips x 10^-4`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FeePair {
    pub mint: UFixValue64,
    pub redeem: UFixValue64,
}

/// Records epoch harvest information for off-chain consumers.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct HarvestCache {
    pub epoch: u64,
    pub stability_pool_cap: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct LevercoinFees {
    pub normal: FeePair,
    pub sell_zone_1: FeePair,
    pub sell_zone_2: FeePair,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PauseEvent {}

/// Outstanding hyUSD debt owed to the earn pool after a Depeg absorption.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PoolDrawdown {
    pub ledger: VirtualStablecoin,
}

/// Floor/ceil deviation percentages for rebalance price curve construction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RebalanceCurveConfig {
    pub floor_pct: UFixValue64,
    pub ceil_pct: UFixValue64,
}

/// Client specified slippage tolerance paired with expected token amount.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SlippageConfig {
    pub expected_token_out: UFixValue64,
    pub slippage_tolerance: UFixValue64,
}

/// **Deprecated** — retained only for `Hylo` account deserialization.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct StablecoinFees {
    pub normal: FeePair,
    pub mode_1: FeePair,
}

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TotalSolCache {
    pub current_update_epoch: u64,
    pub total_sol: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UnpauseEvent {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateDepositLimitEvent {
    pub old_deposit_limit: UFixValue64,
    pub new_deposit_limit: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateWithdrawalFeeEvent {
    pub old_withdrawal_fee: UFixValue64,
    pub new_withdrawal_fee: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateWithdrawalLimitEvent {
    pub old_withdrawal_limit: UFixValue64,
    pub new_withdrawal_limit: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserDepositEvent {
    pub stablecoin_deposited: UFixValue64,
    pub lp_token_nav: UFixValue64,
    pub lp_token_minted: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserWithdrawEvent {
    pub lp_token_burned: UFixValue64,
    pub stablecoin_withdrawn: UFixValue64,
    pub stablecoin_fees: UFixValue64,
}

/// Simple counter representing the supply of a "virtual" stablecoin.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct VirtualStablecoin {
    pub supply: UFixValue64,
}

/// Per-epoch withdrawal window, reset lazily on epoch rollover.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct WithdrawalLimiter {
    pub limit: UFixValue64,
    pub withdrawal_ledger: VirtualStablecoin,
    pub epoch: u64,
}

/// Captures yield harvest configuration as two basis point values:
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct YieldHarvestConfig {
    pub allocation: UFixValue64,
    pub fee: UFixValue64,
}
