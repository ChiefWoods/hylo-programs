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
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [ADDRESS_UPDATE_PROPOSAL, &[address_field.clone() as u8]],
        bump,
        constraint = proposal.load()?.address_field == address_field,
        close = admin,
    )]
    pub proposal: AccountLoader<'info, AddressUpdateProposal>,
}

pub fn handler(
    ctx: Context<CancelAddressUpdate>,
    address_field: AddressField,
) -> Result<CancelAddressUpdateEvent> {
    let proposal = ctx.accounts.proposal.load()?;

    let event = CancelAddressUpdateEvent {
        address_field,
        new_address: proposal.new_address,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
