use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};
use hylo_exchange::constants::{FEE_AUTH, HYLO, HYUSD};

#[derive(Accounts)]
pub struct UserWithdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        seeds = [HYLO],
        bump,
        seeds::program = hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        seeds = [HYUSD],
        bump,
        seeds::program = hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
        seeds::program = hylo_exchange::ID,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_lp_token_ta: UncheckedAccount<'info>,
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
    #[account(mut, seeds = [STAKED_HYUSD], bump)]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UserWithdraw>,
    amount_lp_token: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<UserWithdrawEvent> {
    let _ = (ctx, amount_lp_token, slippage_config);
    todo!()
}
