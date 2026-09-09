use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use hylo_core::pyth::{SOL_USD, USDC_USD};

use crate::{
    constants::*,
    hylo_earn_pool::{accounts::PoolConfig, constants::POOL_CONFIG},
};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct SwapUsdcToLst<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        seeds = [&POOL_CONFIG],
        bump,
        seeds::program = HYLO_EARN_POOL
    )]
    pub pool_config: Account<'info, PoolConfig>,
    #[account(
        has_one = pool_state,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_header: Account<'info, LstHeader>,
    /// CHECK: IDL metadata: relations=lst_header.
    pub pool_state: UncheckedAccount<'info>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [MINT_AUTH, stablecoin_mint.key().as_ref()],
        bump = hylo.stablecoin_auth_bump,
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
        bump = usdc_pair.vault_auth_bump,
    )]
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [POOL_AUTH],
        bump = pool_config.pool_auth_bump,
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
    pub lst_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = usdc_vault_auth,
        associated_token::token_program = token_program,
    )]
    pub usdc_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = pool_auth,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_pool: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = lst_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_lst_ta: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub user_usdc_ta: Account<'info, TokenAccount>,
    pub lst_mint: Account<'info, Mint>,
    #[account(address = anchor_spl::mint::USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut, seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
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
    ctx: Context<SwapUsdcToLst>,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<SwapUsdcToLstEvent> {
    if SOL_USD.address != ctx.accounts.sol_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    if USDC_USD.address != ctx.accounts.usdc_usd_pyth_feed.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }
    let _ = (amount, slippage_config);
    todo!()
}
