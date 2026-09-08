use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeMints<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"stablecoin_mint"}]}.
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"levercoin_mint"}]}.
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,85,83,68]}]}.
    #[account(mut)]
    pub stablecoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[120,83,79,76]}]}.
    #[account(mut)]
    pub levercoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub stablecoin_metadata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s.
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: IDL metadata: address=ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL.
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=SysvarRent111111111111111111111111111111111.
    pub rent: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMints>,
    stablecoin_metadata: TokenMetadata,
    levercoin_metadata: TokenMetadata,
) -> Result<()> {
    let _ = (ctx, stablecoin_metadata, levercoin_metadata);
    todo!()
}
