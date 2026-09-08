use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateOracleConfTolerance<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateOracleConfTolerance>,
    new_oracle_conf_tolerance: UFixValue64,
) -> Result<UpdateOracleConfEvent> {
    let _ = (ctx, new_oracle_conf_tolerance);
    todo!()
}
