use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstStablecoinMintThreshold<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateLstStablecoinMintThreshold>,
    new_stablecoin_mint_threshold: UFixValue64,
) -> Result<UpdateStablecoinMintThresholdEvent> {
    let _ = (ctx, new_stablecoin_mint_threshold);
    todo!()
}
