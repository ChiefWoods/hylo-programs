use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstRebalanceFee<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [LST_HEADER, lst_mint.key().as_ref()],
        bump,
    )]
    pub lst_header: Account<'info, LstHeader>,
    pub lst_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateLstRebalanceFee>,
    new_rebalance_fee: UFixValue64,
) -> Result<UpdateLstRebalanceFeeEvent> {
    let _ = (ctx, new_rebalance_fee);
    todo!()
}
