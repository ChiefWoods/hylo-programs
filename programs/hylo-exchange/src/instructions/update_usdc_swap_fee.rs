use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateUsdcSwapFee<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(
    ctx: Context<UpdateUsdcSwapFee>,
    new_swap_fee: UFixValue64,
) -> Result<UpdateSwapFeeEvent> {
    let _ = (ctx, new_swap_fee);
    todo!()
}
