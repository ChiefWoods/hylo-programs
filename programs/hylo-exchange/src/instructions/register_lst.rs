use anchor_lang::prelude::*;
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
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
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
    #[account(seeds = [LST_REGISTRY_AUTH], bump)]
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
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub lst_stake_pool_state: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sanctum_calculator_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sanctum_calculator_state: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub stake_pool_program_data: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub stake_pool_program: UncheckedAccount<'info>,
    /// CHECK: Address Lookup Table program ID is constrained below.
    #[account(address = solana_sdk_ids::address_lookup_table::ID)]
    pub lut_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterLst>, rebalance_fee: UFixValue64) -> Result<RegisterLstEvent> {
    let _ = (ctx, rebalance_fee);
    todo!()
}
