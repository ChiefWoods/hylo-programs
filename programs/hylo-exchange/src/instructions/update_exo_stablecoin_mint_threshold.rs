use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateExoStablecoinMintThreshold<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(
    ctx: Context<UpdateExoStablecoinMintThreshold>,
    new_stablecoin_mint_threshold: UFixValue64,
) -> Result<UpdateStablecoinMintThresholdEvent> {
    let pair = &mut ctx.accounts.exo_pair.load_mut()?;
    let old_stablecoin_mint_threshold = pair.stablecoin_mint_threshold;
    pair.update_stablecoin_mint_threshold(new_stablecoin_mint_threshold)?;
    let event = UpdateStablecoinMintThresholdEvent {
        old_stablecoin_mint_threshold,
        new_stablecoin_mint_threshold: pair.stablecoin_mint_threshold,
    };
    emit_cpi!(event.clone());
    Ok(event)
}
