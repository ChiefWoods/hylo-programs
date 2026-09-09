use crate::constants::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use hylo_core::asset_swap_config::AssetSwapConfig;
use hylo_core::pyth::{validate_conf_tolerance, validate_interval_secs, USDC_USD};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeUsdc<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        init,
        payer = admin,
        space = UsdcPair::DISCRIMINATOR.len() + UsdcPair::INIT_SPACE,
        seeds = [USDC_PAIR],
        bump,
    )]
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
    mint_fee: UFixValue64,
    redeem_fee: UFixValue64,
    oracle_interval_secs: u64,
    oracle_conf_tolerance: UFixValue64,
    par_tolerance: UFixValue64,
) -> Result<InitializeUsdcEvent> {
    require_keys_eq!(
        ctx.accounts.usdc_usd_pyth_feed.key(),
        USDC_USD.address,
        ErrorCode::ExoOracleInvalid
    );

    let mint_fee = AssetSwapConfig::validate_fee(mint_fee)?;
    let redeem_fee = AssetSwapConfig::validate_fee(redeem_fee)?;
    require!(
        mint_fee != redeem_fee,
        hylo_core::error::CoreError::InvalidFees
    );
    let oracle_interval_secs = validate_interval_secs(oracle_interval_secs)?;
    let oracle_conf_tolerance = validate_conf_tolerance(oracle_conf_tolerance)?;
    let par_tolerance = ParTolerance::validated(par_tolerance)?;

    ctx.accounts.usdc_pair.set_inner(UsdcPair {
        vault_auth_bump: ctx.bumps.usdc_vault_auth,
        fee_auth_bump: ctx.bumps.usdc_fee_auth,
        mint_fee,
        oracle_interval_secs,
        oracle_conf_tolerance,
        virtual_stablecoin: VirtualStablecoin::new(),
        paused: false,
        par_tolerance,
        redeem_fee,
        _reserved: [0; 109],
    });

    let event = InitializeUsdcEvent {
        vault_auth_bump: ctx.bumps.usdc_vault_auth,
        fee_auth_bump: ctx.bumps.usdc_fee_auth,
        mint_fee,
        redeem_fee,
        oracle_interval_secs,
        oracle_conf_tolerance,
        par_tolerance: par_tolerance.tolerance,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
