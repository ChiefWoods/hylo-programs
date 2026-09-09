use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
#[instruction(address_field: AddressField)]
pub struct AcceptAddressUpdate<'info> {
    pub new_address: Signer<'info>,
    /// CHECK: Current admin; receives rent from the closed proposal.
    #[account(mut)]
    pub admin: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        has_one = new_address,
        seeds = [ADDRESS_UPDATE_PROPOSAL, &[address_field.clone() as u8]],
        bump,
        constraint = proposal.load()?.address_field == address_field,
        close = admin,
    )]
    pub proposal: AccountLoader<'info, AddressUpdateProposal>,
}

pub fn handler(
    ctx: Context<AcceptAddressUpdate>,
    address_field: AddressField,
) -> Result<AcceptAddressUpdateEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;
    let proposal = ctx.accounts.proposal.load()?;

    proposal.require_approved_and_live(Clock::get()?.unix_timestamp)?;

    let old_address = hylo.get_address(address_field.clone());
    let new_address = proposal.new_address;
    hylo.set_address(address_field.clone(), new_address)?;

    let event = AcceptAddressUpdateEvent {
        address_field,
        old_address,
        new_address,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
