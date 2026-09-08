use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateLstBuyCurveConfig<'info> {
    /// CHECK: IDL metadata: signer; relations=hylo.
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    #[account(mut)]
    pub hylo: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateLstBuyCurveConfig>,
    new_buy_curve_config: RebalanceCurveConfig,
) -> Result<UpdateRebalanceCurveConfigEvent> {
    let _ = (ctx, new_buy_curve_config);
    todo!()
}
