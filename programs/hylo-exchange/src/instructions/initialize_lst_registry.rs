use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLstRegistry<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[108,115,116,95,114,101,103,105,115,116,114,121,95,97,117,116,104]}]}.
    pub registry_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=AddressLookupTab1e1111111111111111111111111.
    pub lut_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=11111111111111111111111111111111.
    pub system_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<InitializeLstRegistry>, slot: u64) -> Result<()> {
    let _ = (ctx, slot);
    todo!()
}
