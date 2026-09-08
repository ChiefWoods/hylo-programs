use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct PauseLstPair<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub pause_authority: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<PauseLstPair>) -> Result<PauseEvent> {
    let _ = ctx;
    todo!()
}
