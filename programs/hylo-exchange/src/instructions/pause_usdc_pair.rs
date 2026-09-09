use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseUsdcPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(ctx: Context<PauseUsdcPair>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
