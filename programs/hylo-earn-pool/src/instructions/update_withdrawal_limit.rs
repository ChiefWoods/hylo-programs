use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD},
};
#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateWithdrawalLimit<'info> {
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
    ctx: Context<UpdateWithdrawalLimit>,
    new_withdrawal_limit: UFixValue64,
) -> Result<UpdateWithdrawalLimitEvent> {
    let config = &mut ctx.accounts.pool_config.load_mut()?;
    let old_withdrawal_limit = config.withdrawal_limiter.limit;
    config.update_withdrawal_limit(new_withdrawal_limit, Clock::get()?.epoch)?;
    let event = UpdateWithdrawalLimitEvent {
        old_withdrawal_limit,
        new_withdrawal_limit: config.withdrawal_limiter.limit,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
