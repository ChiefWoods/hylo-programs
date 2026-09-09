use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedAdd, UFix64, N9};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::fees::controller::FeeExtract;
use hylo_core::pyth::SOL_USD;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::lst_user_gates;
use crate::instructions::stablecoin_ops::{mint_stablecoin, transfer_user};
use crate::oracle::{load_price_update, oracle_event};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct MintStablecoinLst<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = stablecoin_mint,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.stablecoin_auth_bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_vault: Account<'info, TokenAccount>,
    #[account(seeds = [LST_HEADER, lst_mint.key().as_ref()], bump)]
    pub lst_header: Account<'info, LstHeader>,
    #[account(
        mut,
        token::mint = lst_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_lst_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Account<'info, TokenAccount>,
    pub lst_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<MintStablecoinLst>,
    amount_lst_to_deposit: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<MintStablecoinLstEvent> {
    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    require!(amount_lst_to_deposit > 0, CoreError::ZeroAmount);
    let clock = Clock::get()?;
    let epoch = clock.epoch;
    lst_user_gates(&ctx.accounts.hylo, epoch)?;

    let price_update = load_price_update(&ctx.accounts.sol_usd_pyth_feed, &SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &ctx.accounts.hylo.total_sol_cache,
        ctx.accounts.hylo.stablecoin_mint_threshold()?,
        ctx.accounts.hylo.oracle_config()?,
        ctx.accounts.hylo.levercoin_fees,
        &price_update,
        ctx.accounts.hylo.virtual_stablecoin,
        None,
        ctx.accounts.hylo.lst_sell_curve_config,
        ctx.accounts.hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.stablecoin_mint_enabled(),
        ErrorCode::StablecoinMintDisabled
    );

    let lst_in = UFix64::<N9>::new(amount_lst_to_deposit);
    let price = &ctx.accounts.lst_header.price_sol;
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = exchange.stablecoin_mint_fee(price, lst_in)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    let nav = exchange.stablecoin_nav()?;
    let minted = exchange
        .token_conversion(price)?
        .lst_to_token(amount_remaining, nav)?;
    require!(minted > UFix64::zero(), CoreError::ZeroAmount);
    exchange.validate_stablecoin_amount(minted)?;
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(minted)?;
    }

    let decimals = ctx.accounts.lst_mint.decimals;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_lst_ta.to_account_info(),
        ctx.accounts.lst_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        fees_extracted.bits,
        decimals,
    )?;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.user_lst_ta.to_account_info(),
        ctx.accounts.lst_mint.to_account_info(),
        ctx.accounts.lst_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        amount_remaining.bits,
        decimals,
    )?;
    mint_stablecoin(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.stablecoin_mint.to_account_info(),
        ctx.accounts.user_stablecoin_ta.to_account_info(),
        ctx.accounts.stablecoin_auth.to_account_info(),
        ctx.accounts.stablecoin_mint.key(),
        ctx.accounts.hylo.stablecoin_auth_bump,
        minted.bits,
    )?;

    ctx.accounts.hylo.virtual_stablecoin.mint(minted)?;
    let before = UFix64::<N9>::new(ctx.accounts.lst_vault.amount);
    ctx.accounts.hylo.refresh_lst_vault(
        price,
        before,
        before
            .checked_add(&amount_remaining)
            .ok_or(CoreError::DestinationCollateral)?,
        epoch,
    )?;

    let event = MintStablecoinLstEvent {
        minted: minted.into(),
        nav: nav.into(),
        sol_usd_price: oracle_event(exchange.collateral_oracle_price()),
        lst_mint: ctx.accounts.lst_mint.key(),
        lst_sol_price: price.get_epoch_price(epoch)?.into(),
        collateral_deposited: amount_remaining.into(),
        fees_deposited: fees_extracted.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
