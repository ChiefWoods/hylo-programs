use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::lst_registry;

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
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [LST_REGISTRY_AUTH], bump = hylo.load()?.registry_auth_bump)]
    pub lst_registry_auth: UncheckedAccount<'info>,
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

pub fn handler(ctx: Context<InitializeLstRegistryCalculators>) -> Result<()> {
    let hylo = ctx.accounts.hylo.load()?;

    let registry_data = ctx.accounts.lst_registry.try_borrow_data()?;
    let table = lst_registry::load_table(&registry_data)?;
    let authority = table
        .meta
        .authority
        .ok_or_else(|| error!(ErrorCode::LstRegistryLookupTableDeser))?;
    require_keys_eq!(
        authority,
        ctx.accounts.lst_registry_auth.key(),
        ErrorCode::LstRegistryPreamble
    );
    require!(
        table.addresses.is_empty(),
        ErrorCode::LstRegistryCalculatorsAlreadyInitialized
    );
    drop(registry_data);

    let preamble = lst_registry::calculator_preamble();
    lst_registry::extend_lookup_table(
        ctx.accounts.lut_program.to_account_info(),
        ctx.accounts.lst_registry.to_account_info(),
        ctx.accounts.lst_registry_auth.to_account_info(),
        ctx.accounts.admin.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        hylo.registry_auth_bump,
        &preamble,
    )?;
    Ok(())
}
