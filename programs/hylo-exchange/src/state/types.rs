use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub enum AddressField {
    Admin,
    Treasury,
    PauseAuthority,
}

/// Per-epoch borrow rate for exogenous collateral without native yield.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub struct BorrowRateConfig {
    pub rate: UFixValue64,
    pub fee: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
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

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}
