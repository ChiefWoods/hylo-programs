use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateOracleConfTolerance<'info> {
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
    ctx: Context<UpdateOracleConfTolerance>,
    new_oracle_conf_tolerance: UFixValue64,
) -> Result<UpdateOracleConfEvent> {
    let hylo = &mut ctx.accounts.hylo;
    let old_oracle_conf_tolerance = hylo.oracle_conf_tolerance;
    hylo.update_oracle_conf_tolerance(new_oracle_conf_tolerance)?;
    let event = UpdateOracleConfEvent {
        old_oracle_conf_tolerance,
        new_oracle_conf_tolerance: hylo.oracle_conf_tolerance,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
