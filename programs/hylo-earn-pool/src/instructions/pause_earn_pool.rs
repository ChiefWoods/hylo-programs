use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::{events::*, state::*};
use hylo_exchange::constants::HYLO;

#[derive(Accounts)]
pub struct PauseEarnPool<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        seeds::program = hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
}

pub fn handler(ctx: Context<PauseEarnPool>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
