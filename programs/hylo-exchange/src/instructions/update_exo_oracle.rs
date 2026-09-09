use crate::constants::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use hylo_core::pyth::PythFeed;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoOracle<'info> {
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
    ctx: Context<UpdateExoOracle>,
    new_oracle: Pubkey,
) -> Result<UpdateOracleAddressEvent> {
    let expected = PythFeed::new(ctx.accounts.exo_pair.oracle_feed_id);
    require_keys_eq!(new_oracle, expected.address, ErrorCode::ExoOracleInvalid);
    let pair = &mut ctx.accounts.exo_pair;
    let old_oracle = pair.oracle;
    pair.update_oracle(new_oracle)?;
    let event = UpdateOracleAddressEvent {
        old_oracle,
        new_oracle: pair.oracle,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
