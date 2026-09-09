use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};
use hylo_exchange::constants::{HYLO, HYUSD};

#[derive(Accounts)]
pub struct UpdateDepositLimit<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        seeds::program = hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump)]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(
        seeds = [HYUSD],
        bump,
        seeds::program = hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UpdateDepositLimit>,
    new_deposit_limit: UFixValue64,
) -> Result<UpdateDepositLimitEvent> {
    let _ = (ctx, new_deposit_limit);
    todo!()
}
