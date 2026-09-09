use anchor_lang::prelude::*;

use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLstRegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [LST_REGISTRY_AUTH], bump)]
    pub registry_auth: UncheckedAccount<'info>,
    /// CHECK: Validated owner.
    #[account(
        mut,
        owner = solana_sdk_ids::address_lookup_table::ID
    )]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializeLstRegistry>, slot: u64) -> Result<()> {
    let _ = slot;
    todo!()
}
