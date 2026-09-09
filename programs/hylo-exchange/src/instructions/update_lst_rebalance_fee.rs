use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateLstRebalanceFee<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
        constraint = lst_header.load()?.mint == lst_mint.key(),
    )]
    pub lst_header: AccountLoader<'info, LstHeader>,
    pub lst_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateLstRebalanceFee>,
    new_rebalance_fee: UFixValue64,
) -> Result<UpdateLstRebalanceFeeEvent> {
    let header = &mut ctx.accounts.lst_header.load_mut()?;
    let old_rebalance_fee = header.rebalance_fee;
    header.update_rebalance_fee(new_rebalance_fee)?;
    let event = UpdateLstRebalanceFeeEvent {
        lst_mint: ctx.accounts.lst_mint.key(),
        old_rebalance_fee,
        new_rebalance_fee: header.rebalance_fee,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
