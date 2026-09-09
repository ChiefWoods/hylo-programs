use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseProtocol<'info> {
    pub pause_authority: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<PauseProtocol>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
