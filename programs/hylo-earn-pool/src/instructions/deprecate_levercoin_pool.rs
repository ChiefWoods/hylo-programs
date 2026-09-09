use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, XSOL},
};
#[allow(unused_imports)]
use crate::state::*;

use super::token_ops;

#[derive(Accounts)]
pub struct DeprecateLevercoinPool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump = pool_config.load()?.pool_auth_bump)]
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
        bump = hylo.load()?.levercoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub levercoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DeprecateLevercoinPool>) -> Result<()> {
    let pool_config = ctx.accounts.pool_config.load()?;

    let amount = ctx.accounts.levercoin_pool.amount;
    let pool_auth_bump = [pool_config.pool_auth_bump];
    let pool_auth_seeds: &[&[u8]] = &[POOL_AUTH, &pool_auth_bump];
    token_ops::transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_pool.to_account_info(),
        ctx.accounts.levercoin_mint.to_account_info(),
        ctx.accounts.admin_levercoin_ta.to_account_info(),
        ctx.accounts.pool_auth.to_account_info(),
        amount,
        ctx.accounts.levercoin_mint.decimals,
        pool_auth_seeds,
    )?;
    token_ops::close_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_pool.to_account_info(),
        ctx.accounts.admin.to_account_info(),
        ctx.accounts.pool_auth.to_account_info(),
        pool_auth_seeds,
    )
}
