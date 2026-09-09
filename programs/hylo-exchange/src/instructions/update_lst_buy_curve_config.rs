use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateLstBuyCurveConfig<'info> {
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
    ctx: Context<UpdateLstBuyCurveConfig>,
    new_buy_curve_config: RebalanceCurveConfig,
) -> Result<UpdateRebalanceCurveConfigEvent> {
    let hylo = &mut ctx.accounts.hylo;
    let old_curve_config = hylo.lst_buy_curve_config;
    hylo.update_lst_buy_curve_config(new_buy_curve_config)?;
    let event = UpdateRebalanceCurveConfigEvent {
        old_curve_config,
        new_curve_config: hylo.lst_buy_curve_config,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
