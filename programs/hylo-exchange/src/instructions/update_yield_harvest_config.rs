use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateYieldHarvestConfig<'info> {
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
    ctx: Context<UpdateYieldHarvestConfig>,
    new_yield_harvest_config: YieldHarvestConfig,
) -> Result<UpdateYieldHarvestConfigEvent> {
    let _ = (ctx, new_yield_harvest_config);
    todo!()
}
