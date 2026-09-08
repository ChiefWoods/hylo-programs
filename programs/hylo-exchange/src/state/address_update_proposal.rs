use anchor_lang::prelude::*;

use super::*;

#[account]
pub struct AddressUpdateProposal {
    pub address_field: AddressField,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
    pub approved: bool,
}
