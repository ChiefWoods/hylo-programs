use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateUsdcSwapFee<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[117,115,100,99,95,112,97,105,114]}]}.
    #[account(mut)]
    pub usdc_pair: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateUsdcSwapFee>,
    new_swap_fee: UFixValue64,
) -> Result<UpdateSwapFeeEvent> {
    let _ = (ctx, new_swap_fee);
    todo!()
}
