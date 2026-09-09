use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD},
};
use fix::prelude::{UFix64, N6};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateDepositLimit<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [POOL_AUTH], bump = pool_config.load()?.pool_auth_bump)]
    pub pool_auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(
        seeds = [&HYUSD],
        bump = hylo.load()?.stablecoin_mint_bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UpdateDepositLimit>,
    new_deposit_limit: UFixValue64,
) -> Result<UpdateDepositLimitEvent> {
    let config = &mut ctx.accounts.pool_config.load_mut()?;
    let old_deposit_limit = config.deposit_limiter.limit;
    let pool_amount = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
    config.update_deposit_limit(pool_amount, new_deposit_limit)?;
    let event = UpdateDepositLimitEvent {
        old_deposit_limit,
        new_deposit_limit: config.deposit_limiter.limit,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
