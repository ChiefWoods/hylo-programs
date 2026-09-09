use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct UpdateSolUsdOracle<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateSolUsdOracle>,
    new_oracle: Pubkey,
) -> Result<UpdateOracleAddressEvent> {
    let _ = (ctx, new_oracle);
    todo!()
}
