use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
#[instruction(address_field: AddressField)]
pub struct ProposeAddressUpdate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        init,
        payer = admin,
        space = AddressUpdateProposal::DISCRIMINATOR.len() + AddressUpdateProposal::INIT_SPACE,
        seeds = [ADDRESS_UPDATE_PROPOSAL, &[address_field.clone() as u8]],
        bump,
    )]
    pub proposal: Account<'info, AddressUpdateProposal>,
    /// CHECK: IDL metadata: no additional constraints.
    pub new_address: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ProposeAddressUpdate>,
    address_field: AddressField,
    ttl_secs: u64,
) -> Result<ProposeAddressUpdateEvent> {
    let clock = Clock::get()?;
    let current_address = ctx.accounts.hylo.get_address(address_field.clone());
    ctx.accounts.proposal.init(
        current_address,
        &clock,
        address_field.clone(),
        ctx.accounts.new_address.key(),
        ttl_secs,
    )?;

    let event = ProposeAddressUpdateEvent {
        address_field,
        current_address,
        new_address: ctx.accounts.new_address.key(),
        proposal_time: ctx.accounts.proposal.proposal_time,
        ttl_secs: ctx.accounts.proposal.ttl_secs,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
