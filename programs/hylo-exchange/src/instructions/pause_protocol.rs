use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct PauseProtocol<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(ctx: Context<PauseProtocol>) -> Result<PauseEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    hylo.pause_protocol()?;
    let event = PauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
