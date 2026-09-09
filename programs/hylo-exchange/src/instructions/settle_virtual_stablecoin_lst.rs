use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use hylo_core::pyth::SOL_USD;

use crate::{constants::*, hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG}};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct SettleVirtualStablecoinLst<'info> {
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: Account<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [SETTLEMENT_AUTH], bump)]
    pub settlement_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.stablecoin_auth_bump,
    )]
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump = pool_config.pool_auth_bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<SettleVirtualStablecoinLst>,
) -> Result<SettleVirtualStablecoinLstEvent> {
    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    todo!()
}
