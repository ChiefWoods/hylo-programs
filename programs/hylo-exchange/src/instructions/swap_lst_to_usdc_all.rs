use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use hylo_core::pyth::{SOL_USD, USDC_USD};

use crate::{
    constants::*,
    hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG},
    instructions::rebalance::{self, LstUsdcAccounts},
};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct SwapLstToUsdcAll<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: AccountLoader<'info, PoolConfig>,
    #[account(
        has_one = pool_state,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_header: AccountLoader<'info, LstHeader>,
    /// CHECK: IDL metadata: relations=lst_header.
    pub pool_state: UncheckedAccount<'info>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: AccountLoader<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.load()?.stablecoin_auth_bump,
    )]
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [VAULT_AUTH, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [USDC_VAULT_AUTH, usdc_mint.key().as_ref()],
        bump = usdc_pair.load()?.vault_auth_bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump = pool_config.load()?.pool_auth_bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its fixed seed below.
    #[account(seeds = [SETTLEMENT_AUTH], bump)]
    pub settlement_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = lst_mint,
        associated_token::authority = lst_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub lst_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = lst_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_lst_ta: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_usdc_ta: Box<Account<'info, TokenAccount>>,
    pub lst_mint: Box<Account<'info, Mint>>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(mut, seeds = [HYUSD], bump = hylo.load()?.stablecoin_mint_bump)]
    pub stablecoin_mint: Box<Account<'info, Mint>>,
    /// CHECK: Address is validated against SOL_USD.address in the handler.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: Address is validated against USDC_USD.address in the handler.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Hylo Earn Pool program address is constrained below.
    #[account(address = HYLO_EARN_POOL)]
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    mut ctx: Context<SwapLstToUsdcAll>,
    slippage_config: Option<SlippageConfig>,
) -> Result<()> {
    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    let lst_vault_auth_bump = ctx.bumps.lst_vault_auth;
    let settlement_auth_bump = ctx.bumps.settlement_auth;
    let (swap_event, settle_event) = {
        let a = &mut ctx.accounts;
        rebalance::swap_lst_to_usdc(
            LstUsdcAccounts {
                user: &a.user,
                hylo: &a.hylo,
                pool_config: &a.pool_config,
                lst_header: &a.lst_header,
                pool_state: &a.pool_state,
                usdc_pair: &a.usdc_pair,
                stablecoin_mint_auth: &a.stablecoin_mint_auth,
                lst_vault_auth: &a.lst_vault_auth,
                usdc_vault_auth: &a.usdc_vault_auth,
                pool_auth: &a.pool_auth,
                settlement_auth: &a.settlement_auth,
                lst_vault: &mut a.lst_vault,
                usdc_vault: &mut a.usdc_vault,
                stablecoin_pool: &mut a.stablecoin_pool,
                user_lst_ta: &mut a.user_lst_ta,
                user_usdc_ta: &mut a.user_usdc_ta,
                lst_mint: &a.lst_mint,
                usdc_mint: &a.usdc_mint,
                stablecoin_mint: &mut a.stablecoin_mint,
                sol_usd_pyth_feed: &a.sol_usd_pyth_feed,
                usdc_usd_pyth_feed: &a.usdc_usd_pyth_feed,
                token_program: &a.token_program,
                earn_pool: &a.earn_pool,
                lst_vault_auth_bump,
                settlement_auth_bump,
            },
            None,
            slippage_config,
        )?
    };
    emit_cpi!(swap_event);
    emit_cpi!(settle_event);
    Ok(())
}
