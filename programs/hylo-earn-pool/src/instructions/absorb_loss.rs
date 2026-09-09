use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{UFix64, N6};

#[allow(unused_imports)]
use crate::constants::*;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD, SETTLEMENT_AUTH},
};
use crate::{events::*, state::*};

use super::token_ops;

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
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump = pool_config.load()?.pool_auth_bump)]
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
        bump = hylo.load()?.stablecoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AbsorbLoss>, amount: u64) -> Result<AbsorbLossEvent> {
    let pool_config = ctx.accounts.pool_config.load()?;

    let burned = amount.min(ctx.accounts.stablecoin_pool.amount);
    let remaining = ctx.accounts.stablecoin_pool.amount.saturating_sub(burned);

    let pool_auth_bump = [pool_config.pool_auth_bump];
    let pool_auth_seeds: &[&[u8]] = &[POOL_AUTH, &pool_auth_bump];
    token_ops::burn_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.pool_auth.to_account_info(),
        burned,
        pool_auth_seeds,
    )?;

    Ok(AbsorbLossEvent {
        requested_loss: UFix64::<N6>::new(amount).into(),
        amount_stablecoin_burned: UFix64::<N6>::new(burned).into(),
        remaining_pool_balance: UFix64::<N6>::new(remaining).into(),
    })
}
