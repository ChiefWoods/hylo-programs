use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;
use hylo_core::virtual_stablecoin::VirtualStablecoin;

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}

/// Per-epoch withdrawal window, reset lazily on epoch rollover.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub struct WithdrawalLimiter {
    pub limit: UFixValue64,
    pub withdrawal_ledger: VirtualStablecoin,
    pub epoch: u64,
}
