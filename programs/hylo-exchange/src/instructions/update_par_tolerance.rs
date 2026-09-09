use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateParTolerance<'info> {
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

pub fn handler(
    ctx: Context<UpdateParTolerance>,
    new_par_tolerance: UFixValue64,
) -> Result<UpdateParToleranceEvent> {
    let pair = &mut ctx.accounts.usdc_pair.load_mut()?;
    let old_par_tolerance = pair.par_tolerance.tolerance;
    pair.update_par_tolerance(new_par_tolerance)?;
    let event = UpdateParToleranceEvent {
        old_par_tolerance,
        new_par_tolerance: pair.par_tolerance.tolerance,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
