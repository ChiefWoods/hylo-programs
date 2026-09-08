use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct ProposeAddressUpdate<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[97,100,100,114,101,115,115,95,117,112,100,97,116,101,95,112,114,111,112,111,115,97,108]},{"kind":"arg","path":"address_field"}]}.
    #[account(mut)]
    pub proposal: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub new_address: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=11111111111111111111111111111111.
    pub system_program: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<ProposeAddressUpdate>,
    address_field: AddressField,
    ttl_secs: u64,
) -> Result<ProposeAddressUpdateEvent> {
    let _ = (ctx, address_field, ttl_secs);
    todo!()
}
