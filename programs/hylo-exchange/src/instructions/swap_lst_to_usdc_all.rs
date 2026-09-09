use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct SwapLstToUsdcAll<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [b"pool_config"],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: UncheckedAccount<'info>,
    #[account(
        has_one = pool_state,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_header: Account<'info, LstHeader>,
    /// CHECK: IDL metadata: relations=lst_header.
    pub pool_state: UncheckedAccount<'info>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [SETTLEMENT_AUTH], bump)]
    pub settlement_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = lst_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_lst_ta: UncheckedAccount<'info>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_usdc_ta: Account<'info, TokenAccount>,
    pub lst_mint: Account<'info, Mint>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<SwapLstToUsdcAll>,
    slippage_config: Option<SlippageConfig>,
) -> Result<SwapLstToUsdcEvent> {
    let _ = (ctx, slippage_config);
    todo!()
}
