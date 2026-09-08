use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AddressField {
    Admin,
    Treasury,
    PauseAuthority,
}

/// Per-epoch borrow rate for exogenous collateral without native yield.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct BorrowRateConfig {
    pub rate: UFixValue64,
    pub fee: UFixValue64,
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

/// Captures the true LST price in SOL for the current epoch.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct LstSolPrice {
    pub price: UFixValue64,
    pub epoch: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum LstStakePoolProgram {
    Spl,
    SanctumSpl,
    SanctumSplMulti,
    Marinade,
}

/// Serializable oracle price for event emission.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OraclePriceEvent {
    pub spot: UFixValue64,
    pub conf: UFixValue64,
}

/// Floor/ceil deviation percentages for rebalance price curve construction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RebalanceCurveConfig {
    pub floor_pct: UFixValue64,
    pub ceil_pct: UFixValue64,
}

/// Serializable version of [`RebalancePnl`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum RebalancePnlValue {
    Profit(UFixValue64),
    Loss(UFixValue64),
    NoChange,
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

/// Outstanding hyUSD debt owed to the earn pool after a Depeg absorption.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PoolDrawdown {
    pub ledger: VirtualStablecoin,
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

/// Simple counter representing the supply of a "virtual" stablecoin.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct VirtualStablecoin {
    pub supply: UFixValue64,
}

/// Captures yield harvest configuration as two basis point values:
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct YieldHarvestConfig {
    pub allocation: UFixValue64,
    pub fee: UFixValue64,
}
