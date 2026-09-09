use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateLstStablecoinMintThreshold<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(
    ctx: Context<UpdateLstStablecoinMintThreshold>,
    new_stablecoin_mint_threshold: UFixValue64,
) -> Result<UpdateStablecoinMintThresholdEvent> {
    let hylo = &mut ctx.accounts.hylo.load_mut()?;
    let old_stablecoin_mint_threshold = hylo.stablecoin_mint_threshold;
    hylo.update_stablecoin_mint_threshold(new_stablecoin_mint_threshold)?;
    let event = UpdateStablecoinMintThresholdEvent {
        old_stablecoin_mint_threshold,
        new_stablecoin_mint_threshold: hylo.stablecoin_mint_threshold,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
