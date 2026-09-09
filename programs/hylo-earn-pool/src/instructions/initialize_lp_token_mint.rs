use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::constants::*;

use crate::hylo_exchange::{
    accounts::Hylo,
    constants::{HYLO, MINT_AUTH},
};
#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLpTokenMint<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [POOL_CONFIG], bump)]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [&MINT_AUTH, lp_token_mint.key().as_ref()],
        bump,
    )]
    pub lp_token_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [STAKED_HYUSD], bump)]
    pub lp_token_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub lp_token_metadata: UncheckedAccount<'info>,
    /// CHECK: Metaplex Token Metadata program address is constrained below.
    #[account(address = METAPLEX_TOKEN_METADATA)]
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeLpTokenMint>,
    lp_token_metadata: TokenMetadata,
) -> Result<()> {
    let _ = (ctx, lp_token_metadata);
    todo!()
}
