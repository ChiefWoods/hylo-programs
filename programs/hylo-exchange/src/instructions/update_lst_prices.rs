use anchor_lang::prelude::*;
use fix::prelude::{CheckedAdd, UFix64, N9};

use crate::constants::*;
use crate::error::ErrorCode;
use crate::lst_registry;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
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

pub fn handler(ctx: Context<UpdateLstPrices>) -> Result<UpdateLstPricesEvent> {
    let epoch = Clock::get()?.epoch;
    lst_registry::remaining_matches_table(
        &ctx.accounts.lst_registry.try_borrow_data()?,
        ctx.remaining_accounts,
    )?;

    let (preamble, blocks) = ctx
        .remaining_accounts
        .split_at(LST_REGISTRY_CALCULATOR_PREAMBLE_LEN);

    let mut updated_mints = Vec::new();
    let mut total_sol = UFix64::<N9>::zero();

    for block in blocks.chunks_exact(LST_REGISTRY_BLOCK_LEN) {
        let header_info = &block[0];
        require!(header_info.is_writable, ErrorCode::LstBlockInvalid);
        let mut header = lst_registry::load_header(header_info)?;
        let mint_info = &block[1];
        let vault = lst_registry::load_vault(&block[2])?;
        let pool_state_info = &block[3];

        require_keys_eq!(header.mint, *mint_info.key, ErrorCode::LstBlockInvalid);
        require_keys_eq!(header.vault, *block[2].key, ErrorCode::LstBlockInvalid);
        require_keys_eq!(
            header.pool_state,
            *pool_state_info.key,
            ErrorCode::LstBlockInvalid
        );
        require_keys_eq!(
            *pool_state_info.owner,
            header.stake_program.program_id(),
            ErrorCode::LstBlockInvalid
        );

        let [calculator, calculator_state, stake_program, stake_program_data] =
            lst_registry::calculator_accounts(preamble, &header.stake_program)?;

        let price = lst_registry::lst_to_sol_price(
            calculator.clone(),
            mint_info.clone(),
            calculator_state.clone(),
            pool_state_info.clone(),
            stake_program.clone(),
            stake_program_data.clone(),
        )?;
        header.update_price(price, epoch)?;

        let vault_amount = UFix64::<N9>::new(vault.amount);
        let sol = header.price_sol.convert_lst_to_sol(vault_amount, epoch)?;
        total_sol = total_sol
            .checked_add(&sol)
            .ok_or_else(|| error!(ErrorCode::LstAdditionOverflow))?;

        updated_mints.push(header.mint);
        lst_registry::save_header(header_info, &header)?;
    }

    ctx.accounts.hylo.total_sol_cache.set(total_sol, epoch)?;

    let event = UpdateLstPricesEvent {
        updated_mints,
        new_total_sol: total_sol.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
