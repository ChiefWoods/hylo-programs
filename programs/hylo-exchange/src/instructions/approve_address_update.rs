use anchor_lang::prelude::*;

use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct ApproveAddressUpdate<'info> {
    pub upgrade_authority: Signer<'info>,
    #[account(
        mut,
        has_one = new_address,
    )]
    pub proposal: Account<'info, AddressUpdateProposal>,
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
