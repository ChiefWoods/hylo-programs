use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD, SETTLEMENT_AUTH},
};
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct AbsorbLoss<'info> {
    #[account(
        seeds = [&SETTLEMENT_AUTH],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub settlement_auth: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
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
    #[account(
        mut,
        seeds = [&HYUSD],
        bump = hylo.stablecoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AbsorbLoss>, amount: u64) -> Result<AbsorbLossEvent> {
    let _ = (ctx, amount);
    todo!()
}
