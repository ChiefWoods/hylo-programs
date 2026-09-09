use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use fix::prelude::{UFix64, N6};
use hylo_core::virtual_stablecoin::SUPPLY_FLOOR;

use crate::constants::*;
use crate::error::ErrorCode;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeLstVirtualStablecoin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
        has_one = stablecoin_mint,
    )]
    pub hylo: Account<'info, Hylo>,
    #[account(seeds = [HYUSD], bump = hylo.stablecoin_mint_bump)]
    pub stablecoin_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<InitializeLstVirtualStablecoin>,
) -> Result<InitializeLstVirtualStablecoinEvent> {
    require!(
        ctx.accounts.hylo.virtual_stablecoin.supply()? == UFix64::zero(),
        ErrorCode::LstVirtualStablecoinAlreadyInitialized
    );

    let supply = UFix64::<N6>::new(ctx.accounts.stablecoin_mint.supply);
    require!(supply >= SUPPLY_FLOOR, ErrorCode::TokenAmountPrecisionError);

    ctx.accounts.hylo.virtual_stablecoin.mint(supply)?;

    let event = InitializeLstVirtualStablecoinEvent {
        stablecoin_amount: supply.into(),
    };
    emit_cpi!(event.clone());
    Ok(event)
}
