use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateUsdcOracleConfTolerance<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: AccountLoader<'info, UsdcPair>,
}

pub fn handler(
    ctx: Context<UpdateUsdcOracleConfTolerance>,
    new_oracle_conf_tolerance: UFixValue64,
) -> Result<UpdateOracleConfEvent> {
    let pair = &mut ctx.accounts.usdc_pair.load_mut()?;
    let old_oracle_conf_tolerance = pair.oracle_conf_tolerance;
    pair.update_oracle_conf_tolerance(new_oracle_conf_tolerance)?;
    let event = UpdateOracleConfEvent {
        old_oracle_conf_tolerance,
        new_oracle_conf_tolerance: pair.oracle_conf_tolerance,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
