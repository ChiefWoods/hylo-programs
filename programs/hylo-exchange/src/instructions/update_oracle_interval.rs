use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateOracleInterval<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateOracleInterval>,
    new_oracle_interval_secs: u64,
) -> Result<UpdateOracleIntervalEvent> {
    let _ = (ctx, new_oracle_interval_secs);
    todo!()
}
