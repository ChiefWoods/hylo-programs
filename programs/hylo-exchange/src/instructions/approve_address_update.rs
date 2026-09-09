use anchor_lang::prelude::*;

use crate::{error::ErrorCode, program::HyloExchange};
#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
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
    #[account(
        constraint = hylo_exchange.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_exchange: Program<'info, HyloExchange>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()) @ ErrorCode::AddressChangeUpgradeAuthority
    )]
    pub program_data: Account<'info, ProgramData>,
}

pub fn handler(
    ctx: Context<ApproveAddressUpdate>,
    address_field: AddressField,
) -> Result<ApproveAddressUpdateEvent> {
    todo!()
}
