use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Cannot process deposit yielding zero LP tokens.")]
    ZeroLpDeposit,
    #[msg("Cannot process withdrawal resulting in zero protocol tokens.")]
    ZeroTokenWithdrawal,
    #[msg("Cannot update configuration with identical value.")]
    AdminNoop,
    #[msg("Protocol operations have been paused by admin.")]
    ProtocolPaused,
    #[msg("Earn pool operations have been paused.")]
    EarnPoolPaused,
    #[msg("Earn pool empty due to drawdown. Deposits are temporarily disabled.")]
    DepositDisabled,
}
