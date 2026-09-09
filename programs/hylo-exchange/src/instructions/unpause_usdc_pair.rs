use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UnpauseUsdcPair<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(ctx: Context<UnpauseUsdcPair>) -> Result<UnpauseEvent> {
    let _ = ctx;
    todo!()
}
