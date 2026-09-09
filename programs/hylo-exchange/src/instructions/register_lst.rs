use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable::UpgradeableLoaderState;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{UFix64, N5};
use hylo_core::lst::stake_pool::SplStakePool;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::lst_registry;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct RegisterLst<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = lst_registry,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        init,
        payer = admin,
        space = LstHeader::DISCRIMINATOR.len() + LstHeader::INIT_SPACE,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_header: Account<'info, LstHeader>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [LST_REGISTRY_AUTH], bump = hylo.registry_auth_bump)]
    pub registry_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_vault: Account<'info, TokenAccount>,
    pub lst_mint: Account<'info, Mint>,
    /// CHECK: Validated owner.
    #[account(
        mut,
        owner = solana_sdk_ids::address_lookup_table::ID
    )]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: Validated owner.
    #[account(
        owner = stake_pool_program.key()
    )]
    pub lst_stake_pool_state: UncheckedAccount<'info>,
    /// CHECK: Validated in handler.
    pub sanctum_calculator_program: UncheckedAccount<'info>,
    /// CHECK: Validated owner.
    #[account(
        owner = sanctum_calculator_program.key()
    )]
    pub sanctum_calculator_state: UncheckedAccount<'info>,
    /// CHECK: Validated in handler.
    pub stake_pool_program_data: UncheckedAccount<'info>,
    /// CHECK: Validated in handler.
    pub stake_pool_program: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterLst>, rebalance_fee: UFixValue64) -> Result<RegisterLstEvent> {
    require!(
        ctx.accounts.lst_mint.decimals == LST_DECIMALS,
        ErrorCode::ExoAmountDecimals
    );

    let upgradeable_loader_state = UpgradeableLoaderState::try_deserialize(
        &mut &ctx
            .accounts
            .stake_pool_program
            .to_account_info()
            .data
            .borrow()[..],
    )?;

    match upgradeable_loader_state {
        UpgradeableLoaderState::Program {
            programdata_address,
        } => {
            require_keys_eq!(
                programdata_address,
                ctx.accounts.stake_pool_program_data.key(),
                ErrorCode::LstContextInvalid
            );
        }
        _ => {
            return err!(ErrorCode::LstContextInvalid);
        }
    }

    let lst_stake_pool_program = LstStakePoolProgram::new(ctx.accounts.stake_pool_program.key())
        .ok_or(error!(ErrorCode::LstStakePoolNotSupported))?;

    require_keys_eq!(
        ctx.accounts.sanctum_calculator_program.key(),
        lst_stake_pool_program.calculator(),
        ErrorCode::LstContextInvalid
    );
    require_keys_eq!(
        ctx.accounts.sanctum_calculator_state.key(),
        calculator_state(&lst_stake_pool_program),
        ErrorCode::LstContextInvalid
    );

    let rebalance_fee_typed: UFix64<N5> = rebalance_fee.try_into()?;
    require!(
        rebalance_fee_typed <= UFix64::constant(500),
        hylo_core::error::CoreError::InvalidFees
    );

    {
        let registry_data = ctx.accounts.lst_registry.try_borrow_data()?;
        let table = lst_registry::load_table(&registry_data)?;
        let authority = table
            .meta
            .authority
            .ok_or_else(|| error!(ErrorCode::LstRegistryLookupTableDeser))?;
        require_keys_eq!(
            authority,
            ctx.accounts.registry_auth.key(),
            ErrorCode::LstRegistryPreamble
        );
        require!(
            table.addresses.len() >= LST_REGISTRY_CALCULATOR_PREAMBLE_LEN,
            ErrorCode::LstRegistryPreamble
        );
        require!(
            !table.addresses.contains(&ctx.accounts.lst_mint.key()),
            ErrorCode::LstBlockInvalid
        );
        require!(
            !table
                .addresses
                .contains(&ctx.accounts.lst_stake_pool_state.key()),
            ErrorCode::LstBlockInvalid
        );
    }

    let true_price =
        SplStakePool::from_bytes(&ctx.accounts.lst_stake_pool_state.try_borrow_data()?)?
            .true_price()?;
    let epoch = Clock::get()?.epoch;
    let price_sol = LstSolPrice::new(true_price.price, epoch);

    ctx.accounts.lst_header.set_inner(LstHeader {
        mint: ctx.accounts.lst_mint.key(),
        vault: ctx.accounts.lst_vault.key(),
        pool_state: ctx.accounts.lst_stake_pool_state.key(),
        stake_program: lst_stake_pool_program,
        prev_price_sol: price_sol,
        price_sol,
        last_yield_harvest_epoch: 0,
        rebalance_fee,
        _reserved: [0; 55],
    });

    let block = [
        ctx.accounts.lst_header.key(),
        ctx.accounts.lst_mint.key(),
        ctx.accounts.lst_vault.key(),
        ctx.accounts.lst_stake_pool_state.key(),
    ];
    lst_registry::extend_lookup_table(
        ctx.accounts.lut_program.to_account_info(),
        ctx.accounts.lst_registry.to_account_info(),
        ctx.accounts.registry_auth.to_account_info(),
        ctx.accounts.admin.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.hylo.registry_auth_bump,
        &block,
    )?;

    let event = RegisterLstEvent {
        header: ctx.accounts.lst_header.key(),
        mint: ctx.accounts.lst_mint.key(),
        vault: ctx.accounts.lst_vault.key(),
        pool_state: ctx.accounts.lst_stake_pool_state.key(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}

fn calculator_state(program: &LstStakePoolProgram) -> Pubkey {
    match program {
        LstStakePoolProgram::Spl => SPL_SOL_VALUE_CALCULATOR_STATE,
        LstStakePoolProgram::SanctumSpl => SANCTUM_SPL_SOL_VALUE_CALCULATOR_STATE,
        LstStakePoolProgram::SanctumSplMulti => SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR_STATE,
        LstStakePoolProgram::Marinade => MARINADE_SOL_VALUE_CALCULATOR_STATE,
    }
}
