use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct ConvertStableToLeverLst<'info> {
    /// CHECK: IDL metadata: writable; signer.
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sol_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo; pda={"seeds":[{"kind":"const","value":[104,121,85,83,68]}]}.
    #[account(mut)]
    pub stablecoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"stablecoin_mint"}]}.
    pub stablecoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[102,101,101,95,97,117,116,104]},{"kind":"account","path":"stablecoin_mint"}]}.
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"fee_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"stablecoin_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_stablecoin_ta: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo; pda={"seeds":[{"kind":"const","value":[120,83,79,76]}]}.
    #[account(mut)]
    pub levercoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"levercoin_mint"}]}.
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub user_levercoin_ta: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ConvertStableToLeverLst>,
    amount_stablecoin: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<ConvertStableToLeverLstEvent> {
    let _ = (ctx, amount_stablecoin, slippage_config);
    todo!()
}
