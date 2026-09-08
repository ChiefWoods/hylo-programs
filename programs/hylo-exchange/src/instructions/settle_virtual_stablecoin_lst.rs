use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct SettleVirtualStablecoinLst<'info> {
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,99,111,110,102,105,103]}],"program":{"kind":"const","value":[252,76,145,200,184,154,163,121,164,148,177,58,96,128,21,37,61,78,56,24,51,154,155,244,236,32,127,136,39,150,113,225]}}.
    pub pool_config: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[115,101,116,116,108,101,109,101,110,116,95,97,117,116,104]}]}.
    pub settlement_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"stablecoin_mint"}]}.
    pub stablecoin_mint_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[112,111,111,108,95,97,117,116,104]}],"program":{"kind":"const","value":[252,76,145,200,184,154,163,121,164,148,177,58,96,128,21,37,61,78,56,24,51,154,155,244,236,32,127,136,39,150,113,225]}}.
    pub pool_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"pool_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"stablecoin_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub stablecoin_pool: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,85,83,68]}]}.
    #[account(mut)]
    pub stablecoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA.
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=HysTabVUfmQBFcmzu1ctRd1Y1fxd66RBpboy1bmtDSQQ.
    pub earn_pool: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<SettleVirtualStablecoinLst>,
) -> Result<SettleVirtualStablecoinLstEvent> {
    let _ = ctx;
    todo!()
}
