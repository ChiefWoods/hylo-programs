use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use hylo_core::pyth::USDC_USD;

use crate::{
    constants::*,
    hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG},
    instructions::rebalance::{self, ExoUsdcAccounts},
};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SwapExoToUsdcAll<'info> {
    pub user: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.stablecoin_auth_bump,
    )]
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.vault_auth_bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump = usdc_pair.vault_auth_bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump = pool_config.pool_auth_bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [SETTLEMENT_AUTH], bump)]
    pub settlement_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
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
    #[account(
        mut,
        token::mint = collateral_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_collateral_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_usdc_ta: Account<'info, TokenAccount>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()], bump = exo_pair.levercoin_mint_bump)]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: Address is validated against USDC_USD.address in the handler.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

    pub fn handler(
    mut ctx: Context<SwapExoToUsdcAll>,
    slippage_config: Option<SlippageConfig>,
) -> Result<()> {
    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    let settlement_auth_bump = ctx.bumps.settlement_auth;
    let (swap_event, settle_event) = {
        let a = &mut ctx.accounts;
        rebalance::swap_exo_to_usdc(
            ExoUsdcAccounts {
                user: &a.user,
                hylo: &a.hylo,
                pool_config: &a.pool_config,
                exo_pair: &mut a.exo_pair,
                usdc_pair: &mut a.usdc_pair,
                stablecoin_mint_auth: &a.stablecoin_mint_auth,
                vault_auth: &a.vault_auth,
                usdc_vault_auth: &a.usdc_vault_auth,
                pool_auth: &a.pool_auth,
                settlement_auth: &a.settlement_auth,
                collateral_vault: &mut a.collateral_vault,
                usdc_collateral_vault: &mut a.usdc_collateral_vault,
                stablecoin_pool: &mut a.stablecoin_pool,
                user_collateral_ta: &mut a.user_collateral_ta,
                user_usdc_ta: &mut a.user_usdc_ta,
                collateral_mint: &a.collateral_mint,
                usdc_mint: &a.usdc_mint,
                stablecoin_mint: &mut a.stablecoin_mint,
                levercoin_mint: &a.levercoin_mint,
                collateral_usd_pyth_feed: &a.collateral_usd_pyth_feed,
                usdc_usd_pyth_feed: &a.usdc_usd_pyth_feed,
                token_program: &a.token_program,
                earn_pool: &a.earn_pool,
                settlement_auth_bump,
            },
            None,
            slippage_config,
        )?
    };
    emit_cpi!(swap_event);
    emit_cpi!(settle_event);
    Ok(())
}
