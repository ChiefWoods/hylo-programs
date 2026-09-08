use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct PauseEarnPool<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub pause_authority: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}],"program":{"kind":"const","value":[245,187,72,160,4,116,48,134,197,164,152,189,233,219,27,124,201,65,103,243,58,82,140,90,13,150,83,40,223,158,124,33]}}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,99,111,110,102,105,103]}]}.
    #[account(mut)]
    pub pool_config: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<PauseEarnPool>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
