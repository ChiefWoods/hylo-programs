use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct ConvertStableToLeverLst<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    #[account(mut, seeds = [HYUSD], bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_stablecoin_ta: UncheckedAccount<'info>,
    #[account(mut, seeds = [XSOL], bump)]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_levercoin_ta: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ConvertStableToLeverLst>,
    amount_stablecoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<ConvertStableToLeverLstEvent> {
    let _ = (ctx, amount_stablecoin, slippage_config);
    todo!()
}
