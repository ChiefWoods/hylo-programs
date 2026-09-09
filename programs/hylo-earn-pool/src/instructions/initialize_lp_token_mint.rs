use anchor_lang::prelude::*;
use anchor_spl::metadata::{self, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3};
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
    pub pool_config: AccountLoader<'info, PoolConfig>,
    #[account(
        seeds = [&HYLO],
        bump,
        seeds::program = crate::hylo_exchange::ID,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [&MINT_AUTH, lp_token_mint.key().as_ref()],
        bump,
    )]
    pub lp_token_auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = lp_token_auth,
        seeds = [STAKED_HYUSD],
        bump,
    )]
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
    let pool_config = ctx.accounts.pool_config.load()?;
    require!(
        pool_config.lp_token_mint_bump == 0,
        crate::error::ErrorCode::AdminNoop
    );

    let lp_token_mint_key = ctx.accounts.lp_token_mint.key();
    let lp_token_auth_bump = [ctx.bumps.lp_token_auth];
    let lp_token_auth_signer_seeds: &[&[u8]] =
        &[&MINT_AUTH, lp_token_mint_key.as_ref(), &lp_token_auth_bump];
    metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.lp_token_metadata.to_account_info(),
                mint: ctx.accounts.lp_token_mint.to_account_info(),
                mint_authority: ctx.accounts.lp_token_auth.to_account_info(),
                payer: ctx.accounts.admin.to_account_info(),
                update_authority: ctx.accounts.admin.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[lp_token_auth_signer_seeds],
        ),
        DataV2 {
            name: LP_TOKEN_NAME.to_owned(),
            symbol: lp_token_metadata.symbol,
            uri: lp_token_metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    drop(pool_config);
    let pool_config = &mut ctx.accounts.pool_config.load_mut()?;
    pool_config.lp_token_auth_bump = ctx.bumps.lp_token_auth;
    pool_config.lp_token_mint_bump = ctx.bumps.lp_token_mint;
    Ok(())
}
