use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, TransferChecked};
use fix::prelude::{UFix64, N6};
use hylo_core::asset_swap_config::AssetSwapConfig;
use hylo_core::error::CoreError;
use hylo_core::fees::controller::FeeExtract;
use hylo_core::pyth::{query_pyth_oracle, USDC_USD};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct RedeemStablecoinUsdc<'info> {
    pub user: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = stablecoin_mint,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: AccountLoader<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump = usdc_pair.load()?.vault_auth_bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, usdc_mint.key().as_ref()],
        bump = usdc_pair.load()?.fee_auth_bump,
    )]
    pub usdc_fee_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_collateral_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_fee_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_fee_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = stablecoin_fee_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_fee_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = stablecoin_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_stablecoin_ta: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_usdc_ta: Box<Account<'info, TokenAccount>>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Box<Account<'info, Mint>>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    /// CHECK: Address is validated against USDC_USD.address in the handler.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RedeemStablecoinUsdc>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<RedeemStablecoinUsdcEvent> {
    let hylo = ctx.accounts.hylo.load()?;
    let mut usdc_pair = ctx.accounts.usdc_pair.load_mut()?;

    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    require!(amount > 0, CoreError::ZeroAmount);
    require!(!hylo.protocol_paused, CoreError::ProtocolPaused);
    require!(!usdc_pair.paused, CoreError::PairPaused);

    let clock = Clock::get()?;
    let mut oracle_data: &[u8] = &ctx.accounts.usdc_usd_pyth_feed.try_borrow_data()?;
    let price_update = PriceUpdateV2::try_deserialize(&mut oracle_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    if price_update.price_message.feed_id != USDC_USD.feed_id {
        return Err(ProgramError::InvalidAccountData.into());
    }
    let oracle_price = query_pyth_oracle(&clock, &price_update, usdc_pair.oracle_config()?)?;
    usdc_pair.par_tolerance.validate_spot(oracle_price.spot)?;

    let amount_in = UFix64::<N6>::new(amount);
    let FeeExtract {
        fees_extracted,
        amount_remaining,
    } = AssetSwapConfig::new(usdc_pair.redeem_fee)?.apply_fee(amount_in)?;
    require!(amount_remaining > UFix64::zero(), CoreError::ZeroAmount);
    require!(
        amount_remaining <= usdc_pair.virtual_stablecoin.supply()?,
        CoreError::BurnUnderflow
    );
    require!(
        amount_remaining.bits <= ctx.accounts.usdc_collateral_vault.amount,
        CoreError::InsufficientLiquidity
    );
    if let Some(slippage_config) = slippage_config.as_ref() {
        slippage_config.validate_token_out(amount_remaining)?;
    }

    if fees_extracted > UFix64::zero() {
        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_stablecoin_ta.to_account_info(),
                    mint: ctx.accounts.stablecoin_mint.to_account_info(),
                    to: ctx.accounts.stablecoin_fee_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            fees_extracted.bits,
            ctx.accounts.stablecoin_mint.decimals,
        )?;
    }

    let usdc_mint_key = ctx.accounts.usdc_mint.key();
    let vault_auth_bump = [usdc_pair.vault_auth_bump];
    let vault_auth_seeds: &[&[u8]] = &[USDC_VAULT_AUTH, usdc_mint_key.as_ref(), &vault_auth_bump];
    token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.usdc_collateral_vault.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                to: ctx.accounts.user_usdc_ta.to_account_info(),
                authority: ctx.accounts.usdc_vault_auth.to_account_info(),
            },
            &[vault_auth_seeds],
        ),
        amount_remaining.bits,
        ctx.accounts.usdc_mint.decimals,
    )?;

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.stablecoin_mint.to_account_info(),
                from: ctx.accounts.user_stablecoin_ta.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_remaining.bits,
    )?;

    usdc_pair.virtual_stablecoin.burn(amount_remaining)?;

    let event = RedeemStablecoinUsdcEvent {
        stablecoin_burned: amount_remaining.into(),
        stablecoin_fees: fees_extracted.into(),
        usdc_withdrawn: amount_remaining.into(),
        virtual_stablecoin_supply: usdc_pair.virtual_stablecoin.supply()?.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
