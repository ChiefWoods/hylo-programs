use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
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
    let hylo = &mut ctx.accounts.hylo;
    let old_yield_harvest_config = hylo.yield_harvest_config;
    hylo.update_yield_harvest_config(new_yield_harvest_config)?;
    let event = UpdateYieldHarvestConfigEvent {
        old_yield_harvest_config,
        new_yield_harvest_config: hylo.yield_harvest_config,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
