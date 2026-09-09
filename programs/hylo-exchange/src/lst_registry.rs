use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use solana_address_lookup_table_interface::instruction as alt_ix;
use solana_address_lookup_table_interface::state::AddressLookupTable;

use crate::constants::*;
use crate::error::ErrorCode;

pub fn calculator_preamble() -> [Pubkey; LST_REGISTRY_CALCULATOR_PREAMBLE_LEN] {
    [
        SPL_SOL_VALUE_CALCULATOR,
        SPL_SOL_VALUE_CALCULATOR_STATE,
        SPL_STAKE_POOL_PROGRAM,
        SPL_STAKE_POOL_PROGRAM_DATA,
        SANCTUM_SPL_SOL_VALUE_CALCULATOR,
        SANCTUM_SPL_SOL_VALUE_CALCULATOR_STATE,
        SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM,
        SANCTUM_SPL_STAKE_POOL_PROGRAM_DATA,
        SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR,
        SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR_STATE,
        SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM,
        SANCTUM_SPL_MULTI_STAKE_POOL_PROGRAM_DATA,
        MARINADE_SOL_VALUE_CALCULATOR,
        MARINADE_SOL_VALUE_CALCULATOR_STATE,
        MARINADE_STAKE_POOL_PROGRAM,
        MARINADE_STAKE_POOL_PROGRAM_DATA,
    ]
}

pub fn derive_lst_registry(registry_auth: &Pubkey, slot: u64) -> (Pubkey, u8) {
    alt_ix::derive_lookup_table_address(registry_auth, slot)
}

pub fn load_table(data: &[u8]) -> Result<AddressLookupTable<'_>> {
    AddressLookupTable::deserialize(data)
        .map_err(|_| error!(ErrorCode::LstRegistryLookupTableDeser))
}

pub fn create_lookup_table<'info>(
    lut_program: AccountInfo<'info>,
    lst_registry: AccountInfo<'info>,
    registry_auth: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program_ai: AccountInfo<'info>,
    registry_auth_bump: u8,
    slot: u64,
) -> Result<()> {
    let (instruction, _) = alt_ix::create_lookup_table(*registry_auth.key, *payer.key, slot);
    invoke_lut(
        instruction,
        lut_program,
        lst_registry,
        registry_auth,
        payer,
        system_program_ai,
        registry_auth_bump,
    )
}

pub fn extend_lookup_table<'info>(
    lut_program: AccountInfo<'info>,
    lst_registry: AccountInfo<'info>,
    registry_auth: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program_ai: AccountInfo<'info>,
    registry_auth_bump: u8,
    new_addresses: &[Pubkey],
) -> Result<()> {
    let instruction = alt_ix::extend_lookup_table(
        *lst_registry.key,
        *registry_auth.key,
        Some(*payer.key),
        new_addresses.to_vec(),
    );
    invoke_lut(
        instruction,
        lut_program,
        lst_registry,
        registry_auth,
        payer,
        system_program_ai,
        registry_auth_bump,
    )
}

fn invoke_lut<'info>(
    instruction: anchor_lang::solana_program::instruction::Instruction,
    lut_program: AccountInfo<'info>,
    lst_registry: AccountInfo<'info>,
    registry_auth: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program_ai: AccountInfo<'info>,
    registry_auth_bump: u8,
) -> Result<()> {
    let signer_seeds: &[&[u8]] = &[LST_REGISTRY_AUTH, &[registry_auth_bump]];
    invoke_signed(
        &instruction,
        &[
            lst_registry,
            registry_auth,
            payer,
            system_program_ai,
            lut_program,
        ],
        &[signer_seeds],
    )?;
    Ok(())
}
