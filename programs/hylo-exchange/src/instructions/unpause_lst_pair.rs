use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UnpauseLstPair<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<UnpauseLstPair>) -> Result<UnpauseEvent> {
    ctx.accounts.hylo.unpause_lst_pair()?;
    let event = UnpauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
