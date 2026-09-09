use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{accounts::Hylo, constants::HYLO};
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateWithdrawalFee<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateWithdrawalFee>,
    new_withdrawal_fee: UFixValue64,
) -> Result<UpdateWithdrawalFeeEvent> {
    let config = &mut ctx.accounts.pool_config;
    let old_withdrawal_fee = config.withdrawal_fee;
    config.update_withdrawal_fee(new_withdrawal_fee)?;
    let event = UpdateWithdrawalFeeEvent {
        old_withdrawal_fee,
        new_withdrawal_fee: config.withdrawal_fee,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
