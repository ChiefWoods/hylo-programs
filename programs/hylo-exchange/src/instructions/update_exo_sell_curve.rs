use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoSellCurve<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateExoSellCurve>,
    new_sell_curve_config: RebalanceCurveConfig,
) -> Result<UpdateRebalanceCurveConfigEvent> {
    let pair = &mut ctx.accounts.exo_pair;
    let old_curve_config = pair.sell_curve_config;
    pair.update_sell_curve(new_sell_curve_config)?;
    let event = UpdateRebalanceCurveConfigEvent {
        old_curve_config,
        new_curve_config: pair.sell_curve_config,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
