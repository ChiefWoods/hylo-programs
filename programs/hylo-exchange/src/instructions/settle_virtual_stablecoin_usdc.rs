use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedSub, UFix64, N6, N9};
use hylo_core::exchange_math::total_value_locked;
use hylo_core::pyth::USDC_USD;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG};
use crate::instructions::rebalance::assert_usdc_par;
use crate::instructions::stablecoin_ops::mint_stablecoin;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SettleVirtualStablecoinUsdc<'info> {
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL,
    )]
    pub pool_config: Account<'info, PoolConfig>,
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
        seeds::program = HYLO_EARN_POOL,
    )]
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump = usdc_pair.vault_auth_bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_collateral_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against USDC_USD.address in the handler.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<SettleVirtualStablecoinUsdc>,
) -> Result<SettleVirtualStablecoinUsdcEvent> {
    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let clock = Clock::get()?;
    let usdc_oracle = assert_usdc_par(
        &clock,
        &ctx.accounts.usdc_pair,
        &ctx.accounts.usdc_usd_pyth_feed,
    )?;

    let vault = UFix64::<N6>::new(ctx.accounts.usdc_collateral_vault.amount);
    let vault_n9 = vault
        .checked_convert::<N9>()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))?;
    let tvl = total_value_locked(vault_n9, usdc_oracle.price_range()?.lower)?
        .checked_convert::<N6>()
        .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinConversion))?;
    let virtual_supply = ctx.accounts.usdc_pair.virtual_stablecoin.supply()?;
    require!(tvl > virtual_supply, ErrorCode::SettleVirtualStablecoinNoop);

    let surplus = tvl
        .checked_sub(&virtual_supply)
        .ok_or_else(|| error!(ErrorCode::SettleVirtualStablecoinUnderflow))?;
    require!(surplus > UFix64::zero(), ErrorCode::SettleVirtualStablecoinNoop);

    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.stablecoin_pool.to_account_info(),
        ctx.accounts.stablecoin_mint_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        ctx.accounts.hylo.stablecoin_auth_bump,
        surplus.bits,
    )?;
    ctx.accounts.usdc_pair.virtual_stablecoin.mint(surplus)?;

    let event = SettleVirtualStablecoinUsdcEvent {
        stablecoin_minted: surplus.into(),
        virtual_stablecoin_supply: ctx.accounts.usdc_pair.virtual_stablecoin.supply()?.into(),
        pool_balance: UFix64::<N6>::new(
            ctx.accounts
                .stablecoin_pool
                .amount
                .saturating_add(surplus.bits),
        )
        .into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
