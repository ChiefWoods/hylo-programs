use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
#[instruction(address_field: AddressField)]
pub struct ApproveAddressUpdate<'info> {
    pub upgrade_authority: Signer<'info>,
    #[account(
        mut,
        has_one = new_address,
        seeds = [ADDRESS_UPDATE_PROPOSAL, &[address_field.clone() as u8]],
        bump,
        constraint = proposal.load()?.address_field == address_field,
    )]
    pub proposal: AccountLoader<'info, AddressUpdateProposal>,
    /// CHECK: IDL metadata: relations=proposal.
    pub new_address: UncheckedAccount<'info>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()) @ ErrorCode::AddressChangeUpgradeAuthority
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(
        constraint = hylo_exchange.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_exchange: Program<'info, HyloExchange>,
}

pub fn handler(
    ctx: Context<ApproveAddressUpdate>,
    address_field: AddressField,
) -> Result<ApproveAddressUpdateEvent> {
    let mut proposal = ctx.accounts.proposal.load_mut()?;

    proposal.approve(Clock::get()?.unix_timestamp)?;

    let event = ApproveAddressUpdateEvent {
        address_field,
        new_address: proposal.new_address,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
