use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct PauseLstPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<PauseLstPair>) -> Result<PauseEvent> {
    ctx.accounts.hylo.pause_lst_pair()?;
    let event = PauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
