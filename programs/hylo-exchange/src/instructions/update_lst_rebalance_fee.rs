use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstRebalanceFee<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[108,115,116,95,104,101,97,100,101,114]},{"kind":"account","path":"lst_mint"}]}.
    #[account(mut)]
    pub lst_header: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub lst_mint: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateLstRebalanceFee>,
    new_rebalance_fee: UFixValue64,
) -> Result<UpdateLstRebalanceFeeEvent> {
    let _ = (ctx, new_rebalance_fee);
    todo!()
}
