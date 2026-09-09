use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateExoLevercoinMarketCapLimit<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
    )]
    pub exo_pair: Account<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateExoLevercoinMarketCapLimit>,
    new_levercoin_market_cap_limit: UFixValue64,
) -> Result<UpdateLevercoinMarketCapLimitEvent> {
    let _ = (ctx, new_levercoin_market_cap_limit);
    todo!()
}
