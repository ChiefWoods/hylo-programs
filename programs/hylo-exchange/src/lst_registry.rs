use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::{get_return_data, invoke, invoke_signed};
use fix::prelude::{UFix64, N9};
use solana_address_lookup_table_interface::instruction as alt_ix;
use solana_address_lookup_table_interface::state::AddressLookupTable;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::state::{LstHeader, LstStakePoolProgram};
use anchor_spl::token::TokenAccount;
use inf1_svc_core::instructions::lst_to_sol::LstToSolIxData;
use inf1_svc_core::instructions::{parse_retdata, IX_RETDATA_LEN};

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

/// 1 LST in native 9-decimal atoms, used as the Sanctum `LstToSol` quote size.
pub const LST_TO_SOL_AMOUNT: u64 = 1_000_000_000;

pub fn remaining_matches_table(registry_data: &[u8], remaining: &[AccountInfo]) -> Result<()> {
    let table = load_table(registry_data)?;
    require!(
        remaining.len() == table.addresses.len(),
        ErrorCode::LstRegistryLookupTableInvalid
    );
    for (account, address) in remaining.iter().zip(table.addresses.iter()) {
        require_keys_eq!(
            *account.key,
            *address,
            ErrorCode::LstRegistryLookupTableInvalid
        );
    }
    require!(
        table.addresses.len() >= LST_REGISTRY_CALCULATOR_PREAMBLE_LEN,
        ErrorCode::LstRegistryPreamble
    );
    let blocks_len = table.addresses.len() - LST_REGISTRY_CALCULATOR_PREAMBLE_LEN;
    require!(blocks_len > 0, ErrorCode::LstRegistryEmpty);
    require!(
        blocks_len % LST_REGISTRY_BLOCK_LEN == 0,
        ErrorCode::LstRegistryLookupTableInvalid
    );
    Ok(())
}

pub fn calculator_accounts<'a, 'info>(
    preamble: &'a [AccountInfo<'info>],
    stake_program: &LstStakePoolProgram,
) -> Result<[&'a AccountInfo<'info>; 4]> {
    require!(
        preamble.len() >= LST_REGISTRY_CALCULATOR_PREAMBLE_LEN,
        ErrorCode::LstRegistryPreamble
    );
    let offset = stake_program.preamble_offset();
    let calculator = &preamble[offset];
    let calculator_state = &preamble[offset + 1];
    let program = &preamble[offset + 2];
    let program_data = &preamble[offset + 3];
    require_keys_eq!(
        *calculator.key,
        stake_program.calculator(),
        ErrorCode::LstContextInvalid
    );
    require_keys_eq!(
        *program.key,
        stake_program.program_id(),
        ErrorCode::LstContextInvalid
    );
    Ok([calculator, calculator_state, program, program_data])
}

/// Quotes 1 LST in SOL via the Sanctum calculator. Return data is
/// `(min_lamports, max_lamports)`; the conservative lower bound is the price.
pub fn lst_to_sol_price<'info>(
    calculator: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    calculator_state: AccountInfo<'info>,
    pool_state: AccountInfo<'info>,
    stake_program: AccountInfo<'info>,
    stake_program_data: AccountInfo<'info>,
) -> Result<UFix64<N9>> {
    let calculator_key = *calculator.key;
    let ix = Instruction {
        program_id: calculator_key,
        accounts: vec![
            AccountMeta::new_readonly(*mint.key, false),
            AccountMeta::new_readonly(*calculator_state.key, false),
            AccountMeta::new_readonly(*pool_state.key, false),
            AccountMeta::new_readonly(*stake_program.key, false),
            AccountMeta::new_readonly(*stake_program_data.key, false),
        ],
        data: LstToSolIxData::new(LST_TO_SOL_AMOUNT).as_buf().to_vec(),
    };
    invoke(
        &ix,
        &[
            mint,
            calculator_state,
            pool_state,
            stake_program,
            stake_program_data,
        ],
    )
    .map_err(|_| error!(ErrorCode::SanctumCpi))?;
    let (returned_program, returned) =
        get_return_data().ok_or_else(|| error!(ErrorCode::SanctumCpi))?;
    require_keys_eq!(returned_program, calculator_key, ErrorCode::SanctumCpi);
    require!(returned.len() >= IX_RETDATA_LEN, ErrorCode::SanctumCpi);
    let retdata: [u8; IX_RETDATA_LEN] = returned[..IX_RETDATA_LEN]
        .try_into()
        .map_err(|_| error!(ErrorCode::SanctumCpi))?;
    Ok(UFix64::new(*parse_retdata(&retdata).start()))
}

pub fn load_header(info: &AccountInfo) -> Result<LstHeader> {
    require_keys_eq!(*info.owner, crate::ID, ErrorCode::LstBlockInvalid);
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    LstHeader::try_deserialize(&mut slice).map_err(|_| error!(ErrorCode::LstBlockInvalid))
}

pub fn save_header(info: &AccountInfo, header: &LstHeader) -> Result<()> {
    require!(info.is_writable, ErrorCode::LstBlockInvalid);
    let mut data = info.try_borrow_mut_data()?;
    let disc = LstHeader::DISCRIMINATOR;
    require!(
        data.len() >= disc.len() + core::mem::size_of::<LstHeader>(),
        ErrorCode::LstBlockInvalid
    );
    data[..disc.len()].copy_from_slice(disc.as_ref());
    let body = bytemuck::bytes_of(header);
    data[disc.len()..disc.len() + body.len()].copy_from_slice(body);
    Ok(())
}

pub fn load_vault(info: &AccountInfo) -> Result<TokenAccount> {
    require_keys_eq!(
        *info.owner,
        anchor_spl::token::ID,
        ErrorCode::LstBlockInvalid
    );
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    TokenAccount::try_deserialize(&mut slice).map_err(|_| error!(ErrorCode::LstBlockInvalid))
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
