use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseLstPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<PauseLstPair>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
