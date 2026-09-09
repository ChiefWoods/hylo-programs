use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct MintLevercoinExo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(seeds = [EXO_PAIR, collateral_mint.key().as_ref()], bump)]
    pub exo_pair: Account<'info, ExoPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
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
    pub user_collateral_ta: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_levercoin_ta: UncheckedAccount<'info>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<MintLevercoinExo>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<MintLevercoinExoEvent> {
    let _ = (ctx, amount, slippage_config);
    todo!()
}
