use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use fix::prelude::{CheckedSub, MulDiv, UFix64, N6, N9};
use hylo_core::collateral_ratio::CollateralRatio;
use hylo_core::exchange_math::total_value_locked;
use hylo_core::pyth::query_pyth_oracle;
use hylo_core::rebalance::mode::RebalanceMode;
use hylo_core::util::normalize_mint_exp;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::*;
use crate::error::ErrorCode;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct GenesisMintExo<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: Address is pinned to hylo_idl::pda::DEAD.
    #[account(address = hylo_idl::pda::DEAD)]
    pub dead: UncheckedAccount<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, levercoin_mint.key().as_ref()],
        bump = exo_pair.load()?.levercoin_auth_bump,
    )]
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [EXO_VAULT_AUTH, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.vault_auth_bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub collateral_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = collateral_mint,
        token::authority = admin,
        token::token_program = token_program,
    )]
    pub admin_collateral_ta: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = dead,
        token::token_program = token_program,
    )]
    pub dead_levercoin_ta: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = dead,
        token::token_program = token_program,
    )]
    pub dead_stablecoin_ta: Box<Account<'info, TokenAccount>>,
    pub collateral_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [EXO_LEVERCOIN, collateral_mint.key().as_ref()],
        bump = exo_pair.load()?.levercoin_mint_bump,
    )]
    pub levercoin_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Box<Account<'info, Mint>>,
    /// CHECK: Bound to the pair oracle in the handler.
    pub collateral_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<GenesisMintExo>, amount: u64) -> Result<GenesisMintExoEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

    require!(amount > 0, ErrorCode::ExoGenesisConstraints);
    require!(
        exo_pair.virtual_stablecoin.supply()? == UFix64::zero(),
        ErrorCode::ExoGenesisConstraints
    );
    require!(
        ctx.accounts.levercoin_mint.supply == 0,
        ErrorCode::ExoGenesisConstraints
    );
    require_keys_eq!(
        ctx.accounts.collateral_usd_pyth_feed.key(),
        exo_pair.oracle,
        ErrorCode::ExoOracleInvalid
    );

    let clock = Clock::get()?;
    let mut oracle_data: &[u8] = &ctx.accounts.collateral_usd_pyth_feed.try_borrow_data()?;
    let price_update = PriceUpdateV2::try_deserialize(&mut oracle_data)
        .map_err(|_| error!(ErrorCode::ExoOracleInvalid))?;
    require!(
        price_update.price_message.feed_id == exo_pair.oracle_feed_id,
        ErrorCode::ExoOracleInvalid
    );
    let oracle_price = query_pyth_oracle(&clock, &price_update, exo_pair.oracle_config()?)?;

    let collateral_n9 = normalize_mint_exp(&ctx.accounts.collateral_mint, amount)
        .map_err(|_| error!(ErrorCode::ExoAmountUpConversion))?;
    let tvl_n9 = total_value_locked(collateral_n9, oracle_price.spot)?;
    let target_cr = UFix64::<N9>::new(GENESIS_TARGET_COLLATERAL_RATIO);
    let stablecoin_n9 = tvl_n9
        .mul_div_floor(UFix64::one(), target_cr)
        .ok_or_else(|| error!(ErrorCode::ExoGenesisConstraints))?;
    let stablecoin_minted = stablecoin_n9
        .checked_convert::<N6>()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))?;
    let tvl_n6 = tvl_n9
        .checked_convert::<N6>()
        .ok_or_else(|| error!(ErrorCode::TokenAmountPrecisionError))?;
    let levercoin_minted = tvl_n6
        .checked_sub(&stablecoin_minted)
        .ok_or_else(|| error!(ErrorCode::ExoGenesisConstraints))?;
    require!(
        stablecoin_minted > UFix64::zero() && levercoin_minted > UFix64::zero(),
        ErrorCode::ExoGenesisConstraints
    );

    let collateral_ratio =
        CollateralRatio::new(collateral_n9, oracle_price.spot, stablecoin_minted)?;
    require!(
        RebalanceMode::from_cr(collateral_ratio) == RebalanceMode::Neutral,
        ErrorCode::ExoGenesisCollateralRatio
    );
    let collateral_ratio_value = collateral_ratio
        .as_finite()
        .ok_or_else(|| error!(ErrorCode::ExoGenesisCollateralRatio))?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.admin_collateral_ta.to_account_info(),
                to: ctx.accounts.collateral_vault.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        amount,
    )?;

    let stablecoin_mint_key = ctx.accounts.stablecoin_mint.key();
    let stablecoin_auth_bump = [hylo.stablecoin_auth_bump];
    let stablecoin_auth_seeds: &[&[u8]] = &[
        MINT_AUTH,
        stablecoin_mint_key.as_ref(),
        &stablecoin_auth_bump,
    ];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.stablecoin_mint.to_account_info(),
                to: ctx.accounts.dead_stablecoin_ta.to_account_info(),
                authority: ctx.accounts.stablecoin_auth.to_account_info(),
            },
            &[stablecoin_auth_seeds],
        ),
        stablecoin_minted.bits,
    )?;

    let levercoin_mint_key = ctx.accounts.levercoin_mint.key();
    let levercoin_auth_bump = [exo_pair.levercoin_auth_bump];
    let levercoin_auth_seeds: &[&[u8]] =
        &[MINT_AUTH, levercoin_mint_key.as_ref(), &levercoin_auth_bump];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.levercoin_mint.to_account_info(),
                to: ctx.accounts.dead_levercoin_ta.to_account_info(),
                authority: ctx.accounts.levercoin_auth.to_account_info(),
            },
            &[levercoin_auth_seeds],
        ),
        levercoin_minted.bits,
    )?;

    exo_pair.virtual_stablecoin.mint(stablecoin_minted)?;
    exo_pair.virtual_stablecoin_supply_floor = stablecoin_minted.into();

    let event = GenesisMintExoEvent {
        exo_pair: ctx.accounts.exo_pair.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        collateral_deposited: UFixValue64::new(
            amount,
            -(ctx.accounts.collateral_mint.decimals as i8),
        ),
        levercoin_minted: levercoin_minted.into(),
        stablecoin_minted: stablecoin_minted.into(),
        collateral_ratio: collateral_ratio_value.into(),
        collateral_usd_price: OraclePriceEvent {
            spot: oracle_price.spot.into(),
            conf: oracle_price.conf.into(),
        },
    };
    emit_cpi!(event.clone());
    Ok(event)
}
