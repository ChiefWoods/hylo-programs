use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct AcceptAddressUpdate<'info> {
    /// CHECK: IDL metadata: signer; relations=proposal.
    pub new_address: Signer<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub admin: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[97,100,100,114,101,115,115,95,117,112,100,97,116,101,95,112,114,111,112,111,115,97,108]},{"kind":"arg","path":"address_field"}]}.
    #[account(mut)]
    pub proposal: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<AcceptAddressUpdate>,
    address_field: AddressField,
) -> Result<AcceptAddressUpdateEvent> {
    let _ = (ctx, address_field);
    todo!()
}
