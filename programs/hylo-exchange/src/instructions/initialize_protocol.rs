use anchor_lang::prelude::*;
use crate::constants::*;

use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub upgrade_authority: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: IDL metadata: no additional constraints.
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: IDL metadata: no additional constraints.
    pub program_data: UncheckedAccount<'info>,
    pub hylo_exchange: Program<'info, HyloExchange>,
}

pub fn handler(
    ctx: Context<InitializeProtocol>,
    pause_authority: Pubkey,
    oracle_interval_secs: u64,
    stablecoin_mint_threshold: UFixValue64,
    levercoin_fees: LevercoinFees,
    yield_harvest_config: YieldHarvestConfig,
) -> Result<()> {
    let _ = (
        ctx,
        pause_authority,
        oracle_interval_secs,
        stablecoin_mint_threshold,
        levercoin_fees,
        yield_harvest_config,
    );
    todo!()
}
