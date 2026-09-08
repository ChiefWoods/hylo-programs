use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    /// CHECK: IDL metadata: writable; signer.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[102,101,101,95,97,117,116,104]},{"kind":"account","path":"fee_token_mint"}]}.
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"fee_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"fee_token_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"treasury"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"fee_token_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub treasury_ata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub fee_token_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawFees>) -> Result<WithdrawFeesEvent> {
    let _ = ctx;
    todo!()
}
