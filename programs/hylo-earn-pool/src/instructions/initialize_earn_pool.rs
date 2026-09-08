use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeEarnPool<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: signer.
    pub upgrade_authority: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,99,111,110,102,105,103]}]}.
    #[account(mut)]
    pub pool_config: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}],"program":{"kind":"const","value":[245,187,72,160,4,116,48,134,197,164,152,189,233,219,27,124,201,65,103,243,58,82,140,90,13,150,83,40,223,158,124,33]}}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,97,117,116,104]}]}.
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"pool_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"stablecoin_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub stablecoin_pool: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,85,83,68]}],"program":{"kind":"const","value":[245,187,72,160,4,116,48,134,197,164,152,189,233,219,27,124,201,65,103,243,58,82,140,90,13,150,83,40,223,158,124,33]}}.
    pub stablecoin_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: IDL metadata: no additional constraints.
    pub program_data: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=HysTabVUfmQBFcmzu1ctRd1Y1fxd66RBpboy1bmtDSQQ.
    pub hylo_earn_pool: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<InitializeEarnPool>) -> Result<()> {
    let _ = ctx;
    todo!()
}
