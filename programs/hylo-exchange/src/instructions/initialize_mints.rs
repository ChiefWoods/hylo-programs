use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use std::str::FromStr;

use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeMints<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [HYUSD], bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(mut, seeds = [XSOL], bump)]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub stablecoin_metadata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: Metaplex Token Metadata program address is constrained below.
    #[account(address = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap())]
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMints>,
    stablecoin_metadata: TokenMetadata,
    levercoin_metadata: TokenMetadata,
) -> Result<()> {
    let _ = (ctx, stablecoin_metadata, levercoin_metadata);
    todo!()
}
