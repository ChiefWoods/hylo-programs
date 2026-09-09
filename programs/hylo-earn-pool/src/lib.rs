pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("HysTabVUfmQBFcmzu1ctRd1Y1fxd66RBpboy1bmtDSQQ");
declare_program!(hylo_exchange);

#[program]
pub mod hylo_earn_pool {
    use super::*;

    pub fn absorb_loss(ctx: Context<AbsorbLoss>, amount: u64) -> Result<AbsorbLossEvent> {
        absorb_loss::handler(ctx, amount)
    }

    pub fn deprecate_levercoin_pool(ctx: Context<DeprecateLevercoinPool>) -> Result<()> {
        deprecate_levercoin_pool::handler(ctx)
    }

    pub fn initialize_earn_pool(ctx: Context<InitializeEarnPool>) -> Result<()> {
        initialize_earn_pool::handler(ctx)
    }

    pub fn initialize_lp_token_mint(
        ctx: Context<InitializeLpTokenMint>,
        lp_token_metadata: TokenMetadata,
    ) -> Result<()> {
        initialize_lp_token_mint::handler(ctx, lp_token_metadata)
    }

    pub fn pause_earn_pool(ctx: Context<PauseEarnPool>) -> Result<PauseEvent> {
        pause_earn_pool::handler(ctx)
    }

    pub fn unpause_earn_pool(ctx: Context<UnpauseEarnPool>) -> Result<UnpauseEvent> {
        unpause_earn_pool::handler(ctx)
    }

    pub fn update_deposit_limit(
        ctx: Context<UpdateDepositLimit>,
        new_deposit_limit: UFixValue64,
    ) -> Result<UpdateDepositLimitEvent> {
        update_deposit_limit::handler(ctx, new_deposit_limit)
    }

    pub fn update_withdrawal_fee(
        ctx: Context<UpdateWithdrawalFee>,
        new_withdrawal_fee: UFixValue64,
    ) -> Result<UpdateWithdrawalFeeEvent> {
        update_withdrawal_fee::handler(ctx, new_withdrawal_fee)
    }

    pub fn update_withdrawal_limit(
        ctx: Context<UpdateWithdrawalLimit>,
        new_withdrawal_limit: UFixValue64,
    ) -> Result<UpdateWithdrawalLimitEvent> {
        update_withdrawal_limit::handler(ctx, new_withdrawal_limit)
    }

    pub fn user_deposit(
        ctx: Context<UserDeposit>,
        amount_stablecoin: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<UserDepositEvent> {
        user_deposit::handler(ctx, amount_stablecoin, slippage_config)
    }

    pub fn user_withdraw(
        ctx: Context<UserWithdraw>,
        amount_lp_token: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<UserWithdrawEvent> {
        user_withdraw::handler(ctx, amount_lp_token, slippage_config)
    }
}
