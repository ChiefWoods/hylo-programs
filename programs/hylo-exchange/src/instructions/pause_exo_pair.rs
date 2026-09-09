use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct PauseExoPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(ctx: Context<PauseExoPair>) -> Result<PauseEvent> {
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

    exo_pair.pause()?;
    let event = PauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
