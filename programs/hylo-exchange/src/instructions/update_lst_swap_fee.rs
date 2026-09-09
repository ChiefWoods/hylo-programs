use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateLstSwapFee<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstSwapFee>,
    new_lst_swap_fee: UFixValue64,
) -> Result<UpdateFeeEvent> {
    let hylo = &mut ctx.accounts.hylo;
    let old_fee = hylo.lst_swap_fee;
    hylo.update_lst_swap_fee(new_lst_swap_fee)?;
    let event = UpdateFeeEvent {
        old_fee,
        new_fee: hylo.lst_swap_fee,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
