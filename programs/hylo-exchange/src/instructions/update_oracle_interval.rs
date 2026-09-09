use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateOracleInterval<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateOracleInterval>,
    new_oracle_interval_secs: u64,
) -> Result<UpdateOracleIntervalEvent> {
    let hylo = &mut ctx.accounts.hylo.load_mut()?;
    let old_oracle_interval_secs = hylo.oracle_interval_secs;
    hylo.update_oracle_interval(new_oracle_interval_secs)?;
    let event = UpdateOracleIntervalEvent {
        old_oracle_interval_secs,
        new_oracle_interval_secs: hylo.oracle_interval_secs,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
