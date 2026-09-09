use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstBuyCurveConfig<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstBuyCurveConfig>,
    new_buy_curve_config: RebalanceCurveConfig,
) -> Result<UpdateRebalanceCurveConfigEvent> {
    let _ = (ctx, new_buy_curve_config);
    todo!()
}
