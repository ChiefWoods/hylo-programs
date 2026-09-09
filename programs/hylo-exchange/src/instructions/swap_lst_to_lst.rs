use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::exchange_ops::lst_user_gates;
use crate::instructions::stablecoin_ops::{transfer_pda, transfer_user};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use fix::prelude::{CheckedAdd, CheckedSub, UFix64, N9};
use hylo_core::asset_swap_config::AssetSwapConfig;
use hylo_core::error::CoreError;
use hylo_core::fees::controller::FeeExtract;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SwapLstToLst<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    pub lst_a_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = lst_a_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub lst_a_user_ta: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_a_mint.key().as_ref()],
        bump,
    )]
    pub lst_a_vault_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_a_mint,
        associated_token::authority = lst_a_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_a_vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [LST_HEADER, lst_a_mint.key().as_ref()],
        bump,
    )]
    pub lst_a_header: Account<'info, LstHeader>,
    pub lst_b_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = lst_b_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub lst_b_user_ta: Account<'info, TokenAccount>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_b_mint.key().as_ref()],
        bump,
    )]
    pub lst_b_vault_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_b_mint,
        associated_token::authority = lst_b_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_b_vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [LST_HEADER, lst_b_mint.key().as_ref()],
        bump,
    )]
    pub lst_b_header: Account<'info, LstHeader>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, lst_a_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_a_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<SwapLstToLst>,
    amount_lst_a: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<SwapLstToLstEvent> {
    require!(amount_lst_a > 0, CoreError::ZeroAmount);
    require_keys_neq!(
        ctx.accounts.lst_a_mint.key(),
        ctx.accounts.lst_b_mint.key(),
        ErrorCode::IdentitySwap
    );
    let clock = Clock::get()?;
    let epoch = clock.epoch;
    lst_user_gates(&ctx.accounts.hylo, epoch)?;
    require!(
        ctx.accounts.lst_a_header.price_sol.epoch == epoch
            && ctx.accounts.lst_b_header.price_sol.epoch == epoch,
        ErrorCode::LstPriceOutdated
    );

    let lst_a_in = UFix64::<N9>::new(amount_lst_a);
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = AssetSwapConfig::new(ctx.accounts.hylo.lst_swap_fee)?.apply_fee(lst_a_in)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    let lst_b_out = ctx.accounts.lst_a_header.price_sol.convert_lst_amount(
        epoch,
        amount_remaining,
        &ctx.accounts.lst_b_header.price_sol,
    )?;
    require!(lst_b_out > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        lst_b_out.bits <= ctx.accounts.lst_b_vault.amount,
        CoreError::InsufficientLiquidity
    );
    if let Some(cfg) = slippage_config.as_ref() {
        cfg.validate_token_out(lst_b_out)?;
    }

    let a_decimals = ctx.accounts.lst_a_mint.decimals;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lst_a_user_ta.to_account_info(),
        ctx.accounts.lst_a_mint.to_account_info(),
        ctx.accounts.lst_a_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        amount_remaining.bits,
        a_decimals,
    )?;
    transfer_user(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lst_a_user_ta.to_account_info(),
        ctx.accounts.lst_a_mint.to_account_info(),
        ctx.accounts.fee_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        fees_extracted.bits,
        a_decimals,
    )?;

    let lst_b_mint_key = ctx.accounts.lst_b_mint.key();
    let vault_b_bump = [ctx.bumps.lst_b_vault_auth];
    let vault_b_seeds: &[&[u8]] = &[VAULT_AUTH, lst_b_mint_key.as_ref(), &vault_b_bump];
    transfer_pda(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.lst_b_vault.to_account_info(),
        ctx.accounts.lst_b_mint.to_account_info(),
        ctx.accounts.lst_b_user_ta.to_account_info(),
        ctx.accounts.lst_b_vault_auth.to_account_info(),
        lst_b_out.bits,
        ctx.accounts.lst_b_mint.decimals,
        vault_b_seeds,
    )?;

    let a_before = UFix64::<N9>::new(ctx.accounts.lst_a_vault.amount);
    ctx.accounts.hylo.refresh_lst_vault(
        &ctx.accounts.lst_a_header.price_sol,
        a_before,
        a_before
            .checked_add(&amount_remaining)
            .ok_or(CoreError::DestinationCollateral)?,
        epoch,
    )?;
    let b_before = UFix64::<N9>::new(ctx.accounts.lst_b_vault.amount);
    ctx.accounts.hylo.refresh_lst_vault(
        &ctx.accounts.lst_b_header.price_sol,
        b_before,
        b_before
            .checked_sub(&lst_b_out)
            .ok_or(CoreError::InsufficientLiquidity)?,
        epoch,
    )?;

    let event = SwapLstToLstEvent {
        lst_a_mint: ctx.accounts.lst_a_mint.key(),
        lst_a_in: amount_remaining.into(),
        lst_a_fees_extracted: fees_extracted.into(),
        lst_b_mint: ctx.accounts.lst_b_mint.key(),
        lst_b_out: lst_b_out.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
