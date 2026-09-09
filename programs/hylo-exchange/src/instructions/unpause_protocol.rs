use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UnpauseProtocol<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(ctx: Context<UnpauseProtocol>) -> Result<UnpauseEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    hylo.unpause_protocol()?;
    let event = UnpauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
