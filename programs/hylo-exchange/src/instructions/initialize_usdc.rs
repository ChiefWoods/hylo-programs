use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use hylo_core::pyth::USDC_USD;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct InitializeUsdc<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, usdc_mint.key().as_ref()],
        bump,
    )]
    pub usdc_fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_collateral_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_fee_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_fee_vault: Account<'info, TokenAccount>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against USDC_USD.address in the handler.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeUsdc>,
    swap_fee: UFixValue64,
    oracle_interval_secs: u64,
    oracle_conf_tolerance: UFixValue64,
) -> Result<InitializeUsdcEvent> {
    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    let _ = (swap_fee, oracle_interval_secs, oracle_conf_tolerance);
    todo!()
}
