use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateLevercoinFees<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLevercoinFees>,
    new_levercoin_fees: LevercoinFees,
) -> Result<UpdateLevercoinFeesEvent> {
    let _ = (ctx, new_levercoin_fees);
    todo!()
}
