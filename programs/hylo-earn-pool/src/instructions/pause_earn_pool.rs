use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{accounts::Hylo, constants::HYLO};
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseEarnPool<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = pause_authority,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
}

pub fn handler(ctx: Context<PauseEarnPool>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
