use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
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
    pub hylo: AccountLoader<'info, Hylo>,
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
        init_if_needed,
        payer = payer,
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
    let amount = ctx.accounts.fee_vault.amount;
    if amount > 0 {
        let mint_key = ctx.accounts.fee_token_mint.key();
        let bump = [ctx.bumps.fee_auth];
        let signer_seeds: &[&[u8]] = &[FEE_AUTH, mint_key.as_ref(), &bump];
        token::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.fee_vault.to_account_info(),
                    mint: ctx.accounts.fee_token_mint.to_account_info(),
                    to: ctx.accounts.treasury_ata.to_account_info(),
                    authority: ctx.accounts.fee_auth.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
            ctx.accounts.fee_token_mint.decimals,
        )?;
    }

    let event = WithdrawFeesEvent {
        mint: ctx.accounts.fee_token_mint.key(),
        vault: ctx.accounts.fee_vault.key(),
        treasury_ata: ctx.accounts.treasury_ata.key(),
        amount,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
