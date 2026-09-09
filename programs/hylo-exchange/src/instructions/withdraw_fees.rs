use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = treasury,
    )]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: PDA is constrained by its seeds below.
    #[account(
        seeds = [FEE_AUTH, fee_token_mint.key().as_ref()],
        bump,
    )]
    pub fee_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = fee_token_mint,
        associated_token::authority = fee_auth,
        associated_token::token_program = token_program,
    )]
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = fee_token_mint,
        associated_token::authority = treasury,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata: Account<'info, TokenAccount>,
    pub fee_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawFees>) -> Result<WithdrawFeesEvent> {
    let _ = ctx;
    todo!()
}
