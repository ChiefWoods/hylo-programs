use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{UFix64, N4, N6};
use hylo_core::earn_pool_math::{amount_token_to_withdraw, stablecoin_withdrawal_fee};
use hylo_core::error::CoreError;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{FEE_AUTH, HYLO, HYUSD},
};
#[allow(unused_imports)]
use crate::{events::*, state::*};

use super::token_ops;

#[event_cpi]
#[derive(Accounts)]
pub struct UserWithdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
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
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [&FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
        seeds::program = crate::hylo_exchange::ID,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
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
    #[account(mut, seeds = [STAKED_HYUSD], bump = pool_config.load()?.lp_token_mint_bump)]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UserWithdraw>,
    amount_lp_token: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<UserWithdrawEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;

    require!(!hylo.protocol_paused, ErrorCode::ProtocolPaused);
    require!(!pool_config.paused, ErrorCode::EarnPoolPaused);
    require!(amount_lp_token > 0, CoreError::ZeroAmount);

    let amount_lp = UFix64::<N6>::new(amount_lp_token);
    let lp_supply = UFix64::<N6>::new(ctx.accounts.lp_token_mint.supply);
    let pool_amount = UFix64::<N6>::new(ctx.accounts.stablecoin_pool.amount);
    let gross = amount_token_to_withdraw(amount_lp, lp_supply, pool_amount)?;
    require!(gross > UFix64::zero(), ErrorCode::ZeroTokenWithdrawal);
    require!(
        gross <= pool_amount,
        CoreError::InsufficientEarnPoolLiquidity
    );

    let epoch = Clock::get()?.epoch;
    pool_config
        .withdrawal_limiter
        .validate_withdrawal(gross, epoch)?;

    let withdrawal_fee: UFix64<N4> = pool_config.withdrawal_fee.try_into()?;
    let extract = stablecoin_withdrawal_fee(gross, withdrawal_fee)?;
    require!(
        extract.amount_remaining > UFix64::zero(),
        ErrorCode::ZeroTokenWithdrawal
    );
    if let Some(slippage) = slippage_config {
        slippage.validate_token_out(extract.amount_remaining)?;
    }

    let pool_auth_bump = [pool_config.pool_auth_bump];
    let pool_auth_seeds: &[&[u8]] = &[POOL_AUTH, &pool_auth_bump];
    let decimals = ctx.accounts.stablecoin_mint.decimals;

    token_ops::transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.pool_auth.to_account_info(),
        extract.fees_extracted.bits,
        decimals,
        pool_auth_seeds,
    )?;
    token_ops::transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.pool_auth.to_account_info(),
        extract.amount_remaining.bits,
        decimals,
        pool_auth_seeds,
    )?;
    token_ops::burn_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lp_token_mint.to_account_info(),
        ctx.accounts.user_lp_token_ta.to_account_info(),
        ctx.accounts.user.to_account_info(),
        amount_lp_token,
    )?;

    pool_config
        .withdrawal_limiter
        .register_withdrawal(gross, epoch)?;

    let event = UserWithdrawEvent {
        lp_token_burned: amount_lp.into(),
        stablecoin_withdrawn: extract.amount_remaining.into(),
        stablecoin_fees: extract.fees_extracted.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
