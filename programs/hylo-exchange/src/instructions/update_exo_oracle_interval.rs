use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoOracleInterval<'info> {
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
    ctx: Context<UpdateExoOracleInterval>,
    new_oracle_interval_secs: u64,
) -> Result<UpdateOracleIntervalEvent> {
    let pair = &mut ctx.accounts.exo_pair.load_mut()?;
    let old_oracle_interval_secs = pair.oracle_interval_secs;
    pair.update_oracle_interval(new_oracle_interval_secs)?;
    let event = UpdateOracleIntervalEvent {
        old_oracle_interval_secs,
        new_oracle_interval_secs: pair.oracle_interval_secs,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
