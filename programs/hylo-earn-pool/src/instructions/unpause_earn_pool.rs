use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::{events::*, state::*};
use crate::hylo_exchange::constants::HYLO;

#[derive(Accounts)]
pub struct UnpauseEarnPool<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
}

pub fn handler(ctx: Context<UnpauseEarnPool>) -> Result<UnpauseEvent> {
    let _ = ctx;
    todo!()
}
