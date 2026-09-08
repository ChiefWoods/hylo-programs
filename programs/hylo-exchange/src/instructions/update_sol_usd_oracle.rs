use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateSolUsdOracle<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateSolUsdOracle>,
    new_oracle: Pubkey,
) -> Result<UpdateOracleAddressEvent> {
    let _ = (ctx, new_oracle);
    todo!()
}
