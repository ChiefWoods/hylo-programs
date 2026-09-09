use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseProtocol<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<PauseProtocol>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
