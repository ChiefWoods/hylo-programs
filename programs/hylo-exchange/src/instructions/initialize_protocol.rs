use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ErrorCode;
use crate::program::HyloExchange;
#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub upgrade_authority: Signer<'info>,
    #[account(init, payer = admin, space = Hylo::DISCRIMINATOR.len() + Hylo::INIT_SPACE, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
    /// CHECK: IDL metadata: no additional constraints.
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = hylo_exchange.programdata_address()? == Some(program_data.key())
    )]
    pub hylo_exchange: Program<'info, HyloExchange>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()) @ ErrorCode::AddressChangeUpgradeAuthority
    )]
    pub program_data: Account<'info, ProgramData>,
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
        pause_authority,
        oracle_interval_secs,
        stablecoin_mint_threshold,
        levercoin_fees,
        yield_harvest_config,
    );
    todo!()
}
