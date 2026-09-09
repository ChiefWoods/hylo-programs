use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateLevercoinFees<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLevercoinFees>,
    new_levercoin_fees: LevercoinFees,
) -> Result<UpdateLevercoinFeesEvent> {
    let hylo = &mut ctx.accounts.hylo;
    let old_levercoin_fees = hylo.levercoin_fees;
    hylo.update_levercoin_fees(new_levercoin_fees)?;
    let event = UpdateLevercoinFeesEvent {
        old_levercoin_fees,
        new_levercoin_fees: hylo.levercoin_fees,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
