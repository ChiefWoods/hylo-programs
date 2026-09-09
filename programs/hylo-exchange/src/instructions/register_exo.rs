use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct RegisterExo<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
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
    pub collateral_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub exo_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: Metaplex Token Metadata program address is constrained below.
    #[account(address = METAPLEX_TOKEN_METADATA)]
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterExo>,
    oracle_feed_id: [u8; 32],
    oracle_interval_secs: u64,
    oracle_conf_tolerance: UFixValue64,
    stablecoin_mint_threshold: UFixValue64,
    borrow_rate_config: BorrowRateConfig,
    levercoin_fees: LevercoinFees,
    sell_curve_config: RebalanceCurveConfig,
    buy_curve_config: RebalanceCurveConfig,
    metadata: TokenMetadata,
    levercoin_market_cap_limit: UFixValue64,
) -> Result<RegisterExoEvent> {
    let _ = (
        ctx,
        oracle_feed_id,
        oracle_interval_secs,
        oracle_conf_tolerance,
        stablecoin_mint_threshold,
        borrow_rate_config,
        levercoin_fees,
        sell_curve_config,
        buy_curve_config,
        metadata,
        levercoin_market_cap_limit,
    );
    todo!()
}
