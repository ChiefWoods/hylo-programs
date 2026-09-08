use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeLpTokenMint<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,99,111,110,102,105,103]}]}.
    #[account(mut)]
    pub pool_config: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}],"program":{"kind":"const","value":[245,187,72,160,4,116,48,134,197,164,152,189,233,219,27,124,201,65,103,243,58,82,140,90,13,150,83,40,223,158,124,33]}}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"lp_token_mint"}]}.
    pub lp_token_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[115,116,97,107,101,100,95,104,121,85,83,68]}]}.
    #[account(mut)]
    pub lp_token_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub lp_token_metadata: UncheckedAccount<'info>,
    #[account(address = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap())]
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeLpTokenMint>,
    lp_token_metadata: TokenMetadata,
) -> Result<()> {
    let _ = (ctx, lp_token_metadata);
    todo!()
}
