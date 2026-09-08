use anchor_lang::prelude::*;

use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct ApproveAddressUpdate<'info> {
    /// CHECK: IDL metadata: signer.
    pub upgrade_authority: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[97,100,100,114,101,115,115,95,117,112,100,97,116,101,95,112,114,111,112,111,115,97,108]},{"kind":"arg","path":"address_field"}]}.
    #[account(mut)]
    pub proposal: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: relations=proposal.
    pub new_address: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub program_data: UncheckedAccount<'info>,
    pub hylo_exchange: Program<'info, HyloExchange>,
}

pub fn handler(
    ctx: Context<ApproveAddressUpdate>,
    address_field: AddressField,
) -> Result<ApproveAddressUpdateEvent> {
    let _ = (ctx, address_field);
    todo!()
}
