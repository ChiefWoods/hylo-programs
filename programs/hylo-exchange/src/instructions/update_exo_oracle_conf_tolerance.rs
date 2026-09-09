use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoOracleConfTolerance<'info> {
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
    ctx: Context<UpdateExoOracleConfTolerance>,
    new_oracle_conf_tolerance: UFixValue64,
) -> Result<UpdateOracleConfEvent> {
    let pair = &mut ctx.accounts.exo_pair.load_mut()?;
    let old_oracle_conf_tolerance = pair.oracle_conf_tolerance;
    pair.update_oracle_conf_tolerance(new_oracle_conf_tolerance)?;
    let event = UpdateOracleConfEvent {
        old_oracle_conf_tolerance,
        new_oracle_conf_tolerance: pair.oracle_conf_tolerance,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
