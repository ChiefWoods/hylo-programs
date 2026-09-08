use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateLstSwapFee<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateLstSwapFee>,
    new_lst_swap_fee: UFixValue64,
) -> Result<UpdateSwapFeeEvent> {
    let _ = (ctx, new_lst_swap_fee);
    todo!()
}
