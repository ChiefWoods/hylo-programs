use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UnpauseUsdcPair<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: AccountLoader<'info, UsdcPair>,
}

pub fn handler(ctx: Context<UnpauseUsdcPair>) -> Result<UnpauseEvent> {
    let mut usdc_pair = ctx.accounts.usdc_pair.load_mut()?;

    usdc_pair.unpause()?;
    let event = UnpauseEvent {};
    emit_cpi!(event.clone());
    Ok(event)
}
