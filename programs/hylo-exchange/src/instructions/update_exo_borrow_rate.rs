use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateExoBorrowRate<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateExoBorrowRate>,
    new_borrow_rate_config: BorrowRateConfig,
) -> Result<UpdateExoBorrowRateEvent> {
    let _ = (ctx, new_borrow_rate_config);
    todo!()
}
