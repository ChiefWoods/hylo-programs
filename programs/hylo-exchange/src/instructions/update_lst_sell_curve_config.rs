use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstSellCurveConfig<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstSellCurveConfig>,
    new_sell_curve_config: RebalanceCurveConfig,
) -> Result<UpdateRebalanceCurveConfigEvent> {
    let _ = (ctx, new_sell_curve_config);
    todo!()
}
