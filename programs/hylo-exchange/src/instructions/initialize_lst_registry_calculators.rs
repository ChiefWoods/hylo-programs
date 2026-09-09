use anchor_lang::prelude::*;

use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLstRegistryCalculators<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = lst_registry,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [LST_REGISTRY_AUTH], bump = hylo.registry_auth_bump)]
    pub lst_registry_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeLstRegistryCalculators>) -> Result<()> {
    let _ = ctx;
    todo!()
}
