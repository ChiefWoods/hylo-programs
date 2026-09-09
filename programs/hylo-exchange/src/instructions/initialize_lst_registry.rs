use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::lst_registry;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLstRegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [LST_REGISTRY_AUTH], bump)]
    pub registry_auth: UncheckedAccount<'info>,
    /// CHECK: Created as an Address Lookup Table via CPI; address is derived from registry_auth and slot.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeLstRegistry>, slot: u64) -> Result<()> {
    let hylo = &mut ctx.accounts.hylo.load_mut()?;
    require!(
        hylo.lst_registry == Pubkey::default(),
        ErrorCode::LstRegistryAlreadyInitialized
    );

    let (expected_registry, _) =
        lst_registry::derive_lst_registry(&ctx.accounts.registry_auth.key(), slot);
    require_keys_eq!(
        ctx.accounts.lst_registry.key(),
        expected_registry,
        ErrorCode::LstRegistryLookupTableInvalid
    );

    lst_registry::create_lookup_table(
        ctx.accounts.lut_program.to_account_info(),
        ctx.accounts.lst_registry.to_account_info(),
        ctx.accounts.registry_auth.to_account_info(),
        ctx.accounts.admin.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.bumps.registry_auth,
        slot,
    )?;

    hylo.lst_registry = expected_registry;
    hylo.registry_auth_bump = ctx.bumps.registry_auth;
    Ok(())
}
