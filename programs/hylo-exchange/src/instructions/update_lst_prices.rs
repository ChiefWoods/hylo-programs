use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstPrices<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = lst_registry,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: Validated owner.
    #[account(
        mut,
        owner = solana_sdk_ids::address_lookup_table::ID
    )]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<UpdateLstPrices>) -> Result<UpdateLstPricesEvent> {
    todo!()
}
