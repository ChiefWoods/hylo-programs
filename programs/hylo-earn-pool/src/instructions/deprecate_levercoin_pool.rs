use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;
use crate::hylo_exchange::constants::{HYLO, XSOL};

#[derive(Accounts)]
pub struct DeprecateLevercoinPool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump)]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = levercoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub levercoin_pool: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = admin,
        token::token_program = token_program,
    )]
    pub admin_levercoin_ta: Account<'info, TokenAccount>,
    #[account(
        seeds = [&XSOL],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub levercoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DeprecateLevercoinPool>) -> Result<()> {
    let _ = ctx;
    todo!()
}
