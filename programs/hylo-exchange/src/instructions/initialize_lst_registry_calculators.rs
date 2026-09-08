use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLstRegistryCalculators<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[108,115,116,95,114,101,103,105,115,116,114,121,95,97,117,116,104]}]}.
    pub lst_registry_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=AddressLookupTab1e1111111111111111111111111.
    pub lut_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeLstRegistryCalculators>) -> Result<()> {
    let _ = ctx;
    todo!()
}
