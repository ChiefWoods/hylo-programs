use anchor_lang::prelude::*;
use crate::constants::*;

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
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<UpdateLstPrices>) -> Result<UpdateLstPricesEvent> {
    let _ = ctx;
    todo!()
}
