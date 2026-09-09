use crate::constants::*;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateUsdcMintFee<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(mut, seeds = [USDC_PAIR], bump)]
    pub usdc_pair: Account<'info, UsdcPair>,
}

pub fn handler(
    ctx: Context<UpdateUsdcMintFee>,
    new_mint_fee: UFixValue64,
) -> Result<UpdateFeeEvent> {
    let pair = &mut ctx.accounts.usdc_pair;
    let old_fee = pair.mint_fee;
    pair.update_mint_fee(new_mint_fee)?;
    let event = UpdateFeeEvent {
        old_fee,
        new_fee: pair.mint_fee,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
