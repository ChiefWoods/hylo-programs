use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateUsdcOracleInterval<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(
    ctx: Context<UpdateUsdcOracleInterval>,
    new_oracle_interval_secs: u64,
) -> Result<UpdateOracleIntervalEvent> {
    let pair = &mut ctx.accounts.usdc_pair;
    let old_oracle_interval_secs = pair.oracle_interval_secs;
    pair.update_oracle_interval(new_oracle_interval_secs)?;
    let event = UpdateOracleIntervalEvent {
        old_oracle_interval_secs,
        new_oracle_interval_secs: pair.oracle_interval_secs,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
