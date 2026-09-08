use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct PauseExoPair<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub pause_authority: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[101,120,111,95,112,97,105,114]},{"kind":"account","path":"collateral_mint"}]}.
    #[account(mut)]
    pub exo_pair: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: relations=exo_pair.
    pub collateral_mint: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<PauseExoPair>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
