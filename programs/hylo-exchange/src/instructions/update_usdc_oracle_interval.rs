use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateUsdcOracleInterval<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(
    ctx: Context<UpdateUsdcOracleInterval>,
    new_oracle_interval_secs: u64,
) -> Result<UpdateOracleIntervalEvent> {
    let _ = (ctx, new_oracle_interval_secs);
    todo!()
}
