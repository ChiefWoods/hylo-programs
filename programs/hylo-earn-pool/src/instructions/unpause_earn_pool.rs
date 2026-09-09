use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{accounts::Hylo, constants::HYLO};
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UnpauseEarnPool<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: AccountLoader<'info, PoolConfig>,
}

pub fn handler(ctx: Context<UnpauseEarnPool>) -> Result<UnpauseEvent> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;

    pool_config.unpause()?;
    let event = UnpauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
