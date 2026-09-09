use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct SwapLstToLst<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    pub lst_a_mint: Account<'info, Mint>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub lst_a_user_ta: UncheckedAccount<'info>,
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
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub lst_b_user_ta: UncheckedAccount<'info>,
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
    let _ = (ctx, amount_lst_a, slippage_config);
    todo!()
}
