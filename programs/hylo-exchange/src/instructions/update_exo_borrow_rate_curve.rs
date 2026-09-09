use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoBorrowRateCurve<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateExoBorrowRateCurve>,
    new_curve_config: BorrowRateCurveConfig,
) -> Result<UpdateBorrowRateCurveConfigEvent> {
    let pair = &mut ctx.accounts.exo_pair.load_mut()?;
    let old_curve_config = pair.borrow_rate_curve_config;
    pair.update_borrow_rate_curve(new_curve_config)?;
    let event = UpdateBorrowRateCurveConfigEvent {
        old_curve_config,
        new_curve_config: pair.borrow_rate_curve_config,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
