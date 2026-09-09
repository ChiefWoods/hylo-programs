use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLstStablecoinMintThreshold<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstStablecoinMintThreshold>,
    new_stablecoin_mint_threshold: UFixValue64,
) -> Result<UpdateStablecoinMintThresholdEvent> {
    let _ = (ctx, new_stablecoin_mint_threshold);
    todo!()
}
