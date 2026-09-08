use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct InitializeLstVirtualStablecoin<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: relations=hylo; pda={"seeds":[{"kind":"const","value":[104,121,85,83,68]}]}.
    pub stablecoin_mint: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<InitializeLstVirtualStablecoin>,
) -> Result<InitializeLstVirtualStablecoinEvent> {
    let _ = ctx;
    todo!()
}
