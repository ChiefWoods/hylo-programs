use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
#[instruction(address_field: AddressField)]
pub struct CancelAddressUpdate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [ADDRESS_UPDATE_PROPOSAL, &[address_field.clone() as u8]],
        bump,
        constraint = proposal.address_field == address_field,
        close = admin,
    )]
    pub proposal: Account<'info, AddressUpdateProposal>,
}

pub fn handler(
    ctx: Context<CancelAddressUpdate>,
    address_field: AddressField,
) -> Result<CancelAddressUpdateEvent> {
    let event = CancelAddressUpdateEvent {
        address_field,
        new_address: ctx.accounts.proposal.new_address,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
