use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct PauseExoPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(ctx: Context<PauseExoPair>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
