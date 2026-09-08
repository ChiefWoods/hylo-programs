use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

/// Client specified slippage tolerance paired with expected token amount.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SlippageConfig {
    pub expected_token_out: UFixValue64,
    pub slippage_tolerance: UFixValue64,
}
