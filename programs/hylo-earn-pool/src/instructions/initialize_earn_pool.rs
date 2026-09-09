use crate::program::HyloEarnPool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::UFixValue64;
use hylo_core::limiter::deposit::DepositLimiter;

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
    #[account(
        init,
        payer = admin,
        space = PoolConfig::DISCRIMINATOR.len() + PoolConfig::INIT_SPACE,
        seeds = [POOL_CONFIG],
        bump,
    )]
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
        init_if_needed,
        payer = admin,
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
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(
        constraint = hylo_earn_pool.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_earn_pool: Program<'info, HyloEarnPool>,
}

pub fn handler(ctx: Context<InitializeEarnPool>) -> Result<()> {
    let pool_config = &mut ctx.accounts.pool_config;
    pool_config._dead_admin = ctx.accounts.admin.key();
    pool_config.pool_auth_bump = ctx.bumps.pool_auth;
    pool_config.lp_token_auth_bump = 0;
    pool_config.lp_token_mint_bump = 0;
    pool_config.withdrawal_fee = UFixValue64::new(0, -4);
    pool_config.paused = false;
    pool_config.withdrawal_limiter = WithdrawalLimiter::new(
        UFixValue64::new(u64::MAX, -6),
        hylo_core::virtual_stablecoin::VirtualStablecoin::new(),
        0,
    );
    pool_config.deposit_limiter = DepositLimiter {
        limit: UFixValue64::new(14_999_999_000_000, -6),
    };
    pool_config._reserved = [0; 19];
    Ok(())
}
