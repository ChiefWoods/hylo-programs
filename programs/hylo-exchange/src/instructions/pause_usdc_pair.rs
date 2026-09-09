use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct PauseUsdcPair<'info> {
    pub pause_authority: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = pause_authority,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: AccountLoader<'info, UsdcPair>,
}

pub fn handler(ctx: Context<PauseUsdcPair>) -> Result<PauseEvent> {
    let mut usdc_pair = ctx.accounts.usdc_pair.load_mut()?;

    usdc_pair.pause()?;
    let event = PauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
