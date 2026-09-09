use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{accounts::Hylo, constants::HYLO};
use crate::{events::*, state::*};

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
    let _ = (ctx, new_withdrawal_fee);
    todo!()
}
