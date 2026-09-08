use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("No valid route for given tokens.")]
    UnsupportedPair,
    #[msg("Invalid accounts for route.")]
    InvalidRouteAccounts,
}
