use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};
use crate::hylo_exchange::{accounts::Hylo, constants::{HYLO, HYUSD, MINT_AUTH}};

#[derive(Accounts)]
pub struct UserDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        seeds = [&HYUSD],
        bump = hylo.stablecoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = lp_token_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_lp_token_ta: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump = pool_config.pool_auth_bump)]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [&MINT_AUTH, lp_token_mint.key().as_ref()],
        bump = pool_config.lp_token_auth_bump,
    )]
    pub lp_token_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [STAKED_HYUSD], bump = pool_config.lp_token_mint_bump)]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UserDeposit>,
    amount_stablecoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<UserDepositEvent> {
    let _ = (ctx, amount_stablecoin, slippage_config);
    todo!()
}
