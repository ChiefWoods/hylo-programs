use crate::constants::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{self, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3};
use anchor_spl::token::{Mint, Token, TokenAccount};
use hylo_core::borrow_rate::validate_borrow_rate_fee;
use hylo_core::fees::controller::FeeController;
use hylo_core::limiter::levercoin::validate_levercoin_market_cap_limit;
use hylo_core::pyth::{validate_conf_tolerance, validate_interval_secs, PythFeed};
use hylo_core::rebalance::mode::validate_stablecoin_mint_threshold;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct RegisterExo<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        init,
        payer = admin,
        space = ExoPair::DISCRIMINATOR.len() + core::mem::size_of::<ExoPair>(),
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
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
        init,
        payer = admin,
        mint::decimals = PROTOCOL_TOKEN_DECIMALS,
        mint::authority = levercoin_auth,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump,
    )]
    pub levercoin_mint: Account<'info, Mint>,
    /// CHECK: Validated by the Metaplex metadata CPI below.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: Address is validated against the supplied Pyth feed ID in the handler.
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
    borrow_rate_curve_config: BorrowRateCurveConfig,
    borrow_rate_fee: UFixValue64,
    levercoin_fees: LevercoinFees,
    sell_curve_config: RebalanceCurveConfig,
    buy_curve_config: RebalanceCurveConfig,
    metadata: TokenMetadata,
    levercoin_market_cap_limit: UFixValue64,
) -> Result<RegisterExoEvent> {
    require!(
        (2..=LST_DECIMALS).contains(&ctx.accounts.collateral_mint.decimals),
        ErrorCode::ExoAmountDecimals
    );

    let expected_oracle = PythFeed::new(oracle_feed_id);
    require_keys_eq!(
        ctx.accounts.exo_usd_pyth_feed.key(),
        expected_oracle.address,
        ErrorCode::ExoOracleInvalid
    );

    let oracle_interval_secs = validate_interval_secs(oracle_interval_secs)?;
    let oracle_conf_tolerance = validate_conf_tolerance(oracle_conf_tolerance)?;
    let stablecoin_mint_threshold = validate_stablecoin_mint_threshold(stablecoin_mint_threshold)?;
    let borrow_rate_curve_config = borrow_rate_curve_config.validate()?;
    let borrow_rate_fee = validate_borrow_rate_fee(borrow_rate_fee)?;
    let levercoin_fees = levercoin_fees.validate()?;
    let sell_curve_config = sell_curve_config.validate_sell()?;
    let buy_curve_config = buy_curve_config.validate_buy()?;
    let levercoin_market_cap_limit =
        validate_levercoin_market_cap_limit(levercoin_market_cap_limit)?;

    let current_epoch = Clock::get()?.epoch;
    let mut borrow_rate_harvest_cache = HarvestCache {
        epoch: 0,
        stability_pool_cap: UFixValue64::new(0, -6),
        stablecoin_to_pool: UFixValue64::new(0, -6),
    };
    borrow_rate_harvest_cache.init(current_epoch)?;

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
            name: metadata.symbol.clone(),
            symbol: metadata.symbol,
            uri: metadata.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    *ctx.accounts.exo_pair.load_init()? = ExoPair {
        collateral_mint: ctx.accounts.collateral_mint.key(),
        levercoin_mint_bump: ctx.bumps.levercoin_mint,
        levercoin_auth_bump: ctx.bumps.levercoin_auth,
        vault_auth_bump: ctx.bumps.vault_auth,
        fee_auth_bump: ctx.bumps.fee_auth,
        oracle: ctx.accounts.exo_usd_pyth_feed.key(),
        oracle_feed_id,
        oracle_interval_secs,
        oracle_conf_tolerance,
        stablecoin_mint_threshold,
        virtual_stablecoin: VirtualStablecoin::new(),
        borrow_rate_curve_config,
        borrow_rate_harvest_cache,
        levercoin_fees,
        sell_curve_config,
        buy_curve_config,
        borrow_rate_fee,
        paused: false,
        levercoin_market_cap_limit,
        pool_drawdown: PoolDrawdown::default(),
        virtual_stablecoin_supply_floor: UFixValue64::new(0, -6),
        _reserved: [0; 91],
    };

    let event = RegisterExoEvent {
        exo_pair: ctx.accounts.exo_pair.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        levercoin_mint: levercoin_mint_key,
        collateral_vault: ctx.accounts.collateral_vault.key(),
        fee_vault: ctx.accounts.fee_vault.key(),
        oracle: ctx.accounts.exo_usd_pyth_feed.key(),
        oracle_interval_secs,
        oracle_conf_tolerance,
        borrow_rate_curve_config,
        borrow_rate_fee,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
