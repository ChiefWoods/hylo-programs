use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoLevercoinMarketCapLimit<'info> {
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
    ctx: Context<UpdateExoLevercoinMarketCapLimit>,
    new_levercoin_market_cap_limit: UFixValue64,
) -> Result<UpdateLevercoinMarketCapLimitEvent> {
    let pair = &mut ctx.accounts.exo_pair.load_mut()?;
    let old_levercoin_market_cap_limit = pair.levercoin_market_cap_limit;
    pair.update_levercoin_market_cap_limit(new_levercoin_market_cap_limit)?;
    let event = UpdateLevercoinMarketCapLimitEvent {
        old_levercoin_market_cap_limit,
        new_levercoin_market_cap_limit: pair.levercoin_market_cap_limit,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
