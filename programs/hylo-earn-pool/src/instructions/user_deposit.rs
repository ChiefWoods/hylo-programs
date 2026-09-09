use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{UFix64, N6};
use hylo_core::earn_pool_math::{lp_token_nav, lp_token_out};
use hylo_core::error::CoreError;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, HYUSD, MINT_AUTH},
};
#[allow(unused_imports)]
use crate::{events::*, state::*};

use super::token_ops;

#[event_cpi]
#[derive(Accounts)]
pub struct UserDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [POOL_CONFIG], bump)]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        seeds = [&HYUSD],
        bump = hylo.load()?.stablecoin_mint_bump,
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
    #[account(seeds = [POOL_AUTH], bump = pool_config.load()?.pool_auth_bump)]
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
        bump = pool_config.load()?.lp_token_auth_bump,
    )]
    pub lp_token_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [STAKED_HYUSD], bump = pool_config.load()?.lp_token_mint_bump)]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UserDeposit>,
    amount_stablecoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<UserDepositEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let pool_config = ctx.accounts.pool_config.load()?;

    require!(!hylo.protocol_paused, ErrorCode::ProtocolPaused);
    require!(!pool_config.paused, ErrorCode::EarnPoolPaused);
    require!(amount_stablecoin > 0, CoreError::ZeroAmount);

    let pool_amount = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
    let lp_supply = UFix64::<N6>::new(ctx.accounts.lp_token_mint.supply);
    require!(
        pool_amount > UFix64::zero() || lp_supply == UFix64::zero(),
        ErrorCode::DepositDisabled
    );

    let deposit = UFix64::<N6>::new(amount_stablecoin);
    pool_config
        .deposit_limiter
        .validate_deposit(pool_amount, deposit)?;

    let nav = lp_token_nav(pool_amount, lp_supply)?;
    let lp_out = lp_token_out(deposit, nav)?;
    require!(lp_out > UFix64::zero(), ErrorCode::ZeroLpDeposit);
    if let Some(slippage) = slippage_config {
        slippage.validate_token_out(lp_out)?;
    }

    token_ops::transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.user.to_account_info(),
        amount_stablecoin,
        ctx.accounts.stablecoin_mint.decimals,
    )?;

    let lp_token_mint_key = ctx.accounts.lp_token_mint.key();
    let lp_token_auth_bump = [pool_config.lp_token_auth_bump];
    let lp_token_auth_seeds: &[&[u8]] =
        &[&MINT_AUTH, lp_token_mint_key.as_ref(), &lp_token_auth_bump];
    token_ops::mint_to_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lp_token_mint.to_account_info(),
        ctx.accounts.user_lp_token_ta.to_account_info(),
        ctx.accounts.lp_token_auth.to_account_info(),
        lp_out.bits,
        lp_token_auth_seeds,
    )?;

    let event = UserDepositEvent {
        stablecoin_deposited: deposit.into(),
        lp_token_nav: nav.into(),
        lp_token_minted: lp_out.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
