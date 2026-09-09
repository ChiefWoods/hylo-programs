use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable::UpgradeableLoaderState;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

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
    /// CHECK: Validated address.
    #[account(
        constraint = sanctum_calculator_program.key() == SANCTUM_SPL_SOL_VALUE_CALCULATOR
        || sanctum_calculator_program.key() == SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR
    )]
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
            if programdata_address != ctx.accounts.stake_pool_program_data.key() {
                return Err(ProgramError::InvalidAccountData.into());
            }
        }
        _ => {
            return Err(ProgramError::InvalidAccountData.into());
        }
    }

    let _ = rebalance_fee;
    todo!()
}
