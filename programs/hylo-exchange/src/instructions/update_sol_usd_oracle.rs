use crate::constants::*;
use anchor_lang::prelude::*;
use hylo_core::pyth::SOL_USD;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateSolUsdOracle<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateSolUsdOracle>,
    new_oracle: Pubkey,
) -> Result<UpdateOracleAddressEvent> {
    require_keys_eq!(new_oracle, SOL_USD.address);
    let hylo = &mut ctx.accounts.hylo.load_mut()?;
    let old_oracle = hylo.sol_usd_oracle;
    hylo.update_sol_usd_oracle(new_oracle)?;
    let event = UpdateOracleAddressEvent {
        old_oracle,
        new_oracle: hylo.sol_usd_oracle,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
