use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{self, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3};
use anchor_spl::token::{Mint, Token};

use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeMints<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
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
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = stablecoin_auth,
        seeds = [HYUSD],
        bump,
    )]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = levercoin_auth,
        seeds = [XSOL],
        bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: Validated by the Metaplex metadata CPI below.
    #[account(mut)]
    pub stablecoin_metadata: UncheckedAccount<'info>,
    /// CHECK: Validated by the Metaplex metadata CPI below.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: Metaplex Token Metadata program address is constrained below.
    #[account(address = METAPLEX_TOKEN_METADATA)]
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
    let stablecoin_mint_key = ctx.accounts.stablecoin_mint.key();
    let stablecoin_auth_bump = [ctx.bumps.stablecoin_auth];
    let stablecoin_auth_signer_seeds: &[&[u8]] = &[
        MINT_AUTH,
        stablecoin_mint_key.as_ref(),
        &stablecoin_auth_bump,
    ];
    metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.stablecoin_metadata.to_account_info(),
                mint: ctx.accounts.stablecoin_mint.to_account_info(),
                mint_authority: ctx.accounts.stablecoin_auth.to_account_info(),
                payer: ctx.accounts.admin.to_account_info(),
                update_authority: ctx.accounts.admin.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[stablecoin_auth_signer_seeds],
        ),
        DataV2 {
            name: STABLECOIN_TOKEN_NAME.to_owned(),
            symbol: stablecoin_metadata.symbol,
            uri: stablecoin_metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    let levercoin_mint_key = ctx.accounts.levercoin_mint.key();
    let levercoin_auth_bump = [ctx.bumps.levercoin_auth];
    let levercoin_auth_signer_seeds: &[&[u8]] =
        &[MINT_AUTH, levercoin_mint_key.as_ref(), &levercoin_auth_bump];
    metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.levercoin_metadata.to_account_info(),
                mint: ctx.accounts.levercoin_mint.to_account_info(),
                mint_authority: ctx.accounts.levercoin_auth.to_account_info(),
                payer: ctx.accounts.admin.to_account_info(),
                update_authority: ctx.accounts.admin.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[levercoin_auth_signer_seeds],
        ),
        DataV2 {
            name: LST_LEVERCOIN_TOKEN_NAME.to_owned(),
            symbol: levercoin_metadata.symbol,
            uri: levercoin_metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    let hylo = &mut ctx.accounts.hylo.load_mut()?;
    hylo.stablecoin_mint = stablecoin_mint_key;
    hylo.stablecoin_mint_bump = ctx.bumps.stablecoin_mint;
    hylo.stablecoin_auth_bump = ctx.bumps.stablecoin_auth;
    hylo.levercoin_mint = levercoin_mint_key;
    hylo.levercoin_mint_bump = ctx.bumps.levercoin_mint;
    hylo.levercoin_auth_bump = ctx.bumps.levercoin_auth;

    Ok(())
}
