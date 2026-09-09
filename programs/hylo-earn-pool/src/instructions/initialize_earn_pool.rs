use crate::program::HyloEarnPool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD},
};
#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeEarnPool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub upgrade_authority: Signer<'info>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump)]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(
        seeds = [&HYUSD],
        bump = hylo.stablecoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = hylo_earn_pool.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_earn_pool: Program<'info, HyloEarnPool>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
}

pub fn handler(ctx: Context<InitializeEarnPool>) -> Result<()> {
    todo!()
}
