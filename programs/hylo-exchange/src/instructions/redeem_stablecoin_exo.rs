use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct RedeemStablecoinExo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, collateral_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_stablecoin_ta: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_collateral_ta: UncheckedAccount<'info>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RedeemStablecoinExo>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<RedeemStablecoinExoEvent> {
    let _ = (ctx, amount, slippage_config);
    todo!()
}
