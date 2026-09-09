use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct CancelAddressUpdate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut)]
    pub proposal: Account<'info, AddressUpdateProposal>,
}

pub fn handler(
    ctx: Context<CancelAddressUpdate>,
    address_field: AddressField,
) -> Result<CancelAddressUpdateEvent> {
    let _ = (ctx, address_field);
    todo!()
}
