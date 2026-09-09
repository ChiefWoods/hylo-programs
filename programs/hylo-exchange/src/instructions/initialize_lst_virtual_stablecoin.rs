use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct InitializeLstVirtualStablecoin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<InitializeLstVirtualStablecoin>,
) -> Result<InitializeLstVirtualStablecoinEvent> {
    let _ = ctx;
    todo!()
}
