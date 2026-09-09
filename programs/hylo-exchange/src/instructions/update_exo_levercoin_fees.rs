use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoLevercoinFees<'info> {
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
    ctx: Context<UpdateExoLevercoinFees>,
    new_levercoin_fees: LevercoinFees,
) -> Result<UpdateLevercoinFeesEvent> {
    let pair = &mut ctx.accounts.exo_pair;
    let old_levercoin_fees = pair.levercoin_fees;
    pair.update_levercoin_fees(new_levercoin_fees)?;
    let event = UpdateLevercoinFeesEvent {
        old_levercoin_fees,
        new_levercoin_fees: pair.levercoin_fees,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
