use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstSwapFee<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstSwapFee>,
    new_lst_swap_fee: UFixValue64,
) -> Result<UpdateSwapFeeEvent> {
    let _ = (ctx, new_lst_swap_fee);
    todo!()
}
