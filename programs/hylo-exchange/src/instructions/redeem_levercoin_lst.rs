use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use fix::prelude::{CheckedSub, UFix64, N6, N9};
use hylo_core::error::CoreError;
use hylo_core::exchange_context::{ExchangeContext, LstExchangeContext};
use hylo_core::fees::controller::FeeExtract;
use hylo_core::pyth::SOL_USD;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::lst_user_gates;
use crate::instructions::stablecoin_ops::{burn_tokens, transfer_pda};
use crate::oracle::{load_price_update, oracle_event};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct RedeemLevercoinLst<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = levercoin_mint,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
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
    pub lst_header: AccountLoader<'info, LstHeader>,
    #[account(
        mut,
        token::mint = levercoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_levercoin_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = lst_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_lst_ta: Account<'info, TokenAccount>,
    #[account(mut, seeds = [XSOL], bump = hylo.load()?.levercoin_mint_bump)]
    pub levercoin_mint: Account<'info, Mint>,
    pub lst_mint: Account<'info, Mint>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RedeemLevercoinLst>,
    amount_to_redeem: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<RedeemLevercoinLstEvent> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;
    let lst_header = ctx.accounts.lst_header.load()?;

    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    require!(amount_to_redeem > 0, CoreError::ZeroAmount);
    let clock = Clock::get()?;
    let epoch = clock.epoch;
    lst_user_gates(&hylo, epoch)?;

    let price_update = load_price_update(&ctx.accounts.sol_usd_pyth_feed, &SOL_USD.feed_id)?;
    let exchange = LstExchangeContext::load(
        clock,
        &hylo.total_sol_cache,
        hylo.stablecoin_mint_threshold()?,
        hylo.oracle_config()?,
        hylo.levercoin_fees,
        &price_update,
        hylo.virtual_stablecoin,
        Some(&ctx.accounts.levercoin_mint),
        hylo.lst_sell_curve_config,
        hylo.lst_buy_curve_config,
    )?;
    require!(
        exchange.rebalance_mode() != hylo_core::rebalance::mode::RebalanceMode::Depeg,
        ErrorCode::LevercoinRedeemDisabled
    );

    let redeemed = UFix64::<N6>::new(amount_to_redeem);
    require!(
        redeemed <= exchange.levercoin_supply()?,
        CoreError::ZeroAmount
    );
    let price = &lst_header.price_sol;
    let nav = exchange.levercoin_redeem_nav()?;
    let gross_lst = exchange
        .token_conversion(price)?
        .token_to_lst(redeemed, nav)?;
    require!(gross_lst > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        gross_lst.bits <= ctx.accounts.lst_vault.amount,
        CoreError::InsufficientLiquidity
    );
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = exchange.levercoin_redeem_fee(price, gross_lst)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(amount_remaining)?;
    }

    let lst_mint_key = ctx.accounts.lst_mint.key();
    let vault_bump = [ctx.bumps.vault_auth];
    let vault_seeds: &[&[u8]] = &[VAULT_AUTH, lst_mint_key.as_ref(), &vault_bump];
    let decimals = ctx.accounts.lst_mint.decimals;
    transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lst_vault.to_account_info(),
        ctx.accounts.lst_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.vault_auth.to_account_info(),
        fees_extracted.bits,
        decimals,
        vault_seeds,
    )?;
    transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lst_vault.to_account_info(),
        ctx.accounts.lst_mint.to_account_info(),
        ctx.accounts.user_lst_ta.to_account_info(),
        ctx.accounts.vault_auth.to_account_info(),
        amount_remaining.bits,
        decimals,
        vault_seeds,
    )?;
    burn_tokens(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.levercoin_mint.to_account_info(),
        ctx.accounts.user_levercoin_ta.to_account_info(),
        ctx.accounts.user.to_account_info(),
        redeemed.bits,
    )?;

    let before = UFix64::<N9>::new(ctx.accounts.lst_vault.amount);
    hylo.refresh_lst_vault(
        price,
        before,
        before
            .checked_sub(&gross_lst)
            .ok_or(CoreError::InsufficientLiquidity)?,
        epoch,
    )?;

    let event = RedeemLevercoinLstEvent {
        redeemed: redeemed.into(),
        nav: nav.into(),
        sol_usd_price: oracle_event(exchange.collateral_oracle_price()),
        lst_mint: ctx.accounts.lst_mint.key(),
        lst_sol_price: price.get_epoch_price(epoch)?.into(),
        collateral_withdrawn: amount_remaining.into(),
        fees_deposited: fees_extracted.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
