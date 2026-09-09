use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct AcceptAddressUpdate<'info> {
    pub new_address: Signer<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub admin: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut)]
    pub proposal: Account<'info, AddressUpdateProposal>,
}

pub fn handler(
    ctx: Context<AcceptAddressUpdate>,
    address_field: AddressField,
) -> Result<AcceptAddressUpdateEvent> {
    let _ = (ctx, address_field);
    todo!()
}
