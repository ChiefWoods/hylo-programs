use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct GenesisMintExo<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: Fixed dead account
    #[account(address = Pubkey::from_str("BzaxDc5L5zvnMfXVjpdynRKrXp8uETaUZW6bYigTM2fe").unwrap())]
    pub dead: UncheckedAccount<'info>,
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
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub admin_collateral_ta: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub dead_levercoin_ta: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub dead_stablecoin_ta: UncheckedAccount<'info>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<GenesisMintExo>, amount: u64) -> Result<GenesisMintExoEvent> {
    let _ = (ctx, amount);
    todo!()
}
