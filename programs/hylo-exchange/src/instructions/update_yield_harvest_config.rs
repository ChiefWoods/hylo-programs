use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateYieldHarvestConfig<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateYieldHarvestConfig>,
    new_yield_harvest_config: YieldHarvestConfig,
) -> Result<UpdateYieldHarvestConfigEvent> {
    let _ = (ctx, new_yield_harvest_config);
    todo!()
}
