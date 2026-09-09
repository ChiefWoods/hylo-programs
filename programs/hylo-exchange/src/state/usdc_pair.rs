use anchor_lang::prelude::*;
use hylo_core::{
    asset_swap_config::AssetSwapConfig,
    pyth::{validate_conf_tolerance, validate_interval_secs, OracleConfig},
};

use super::*;

#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct UsdcPair {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub mint_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub paused: bool,
    pub par_tolerance: ParTolerance,
    pub redeem_fee: UFixValue64,
    pub _reserved: [u8; 109],
}

impl UsdcPair {
    pub fn oracle_config(&self) -> Result<OracleConfig> {
        Ok(OracleConfig::new(
            self.oracle_interval_secs,
            self.oracle_conf_tolerance.try_into()?,
        ))
    }

    pub fn swap_oracle_config(&self) -> Result<OracleConfig> {
        self.oracle_config()
    }

    pub fn update_oracle_interval(&mut self, new_interval: u64) -> Result<()> {
        let new_interval = validate_interval_secs(new_interval)?;
        require!(
            self.oracle_interval_secs != new_interval,
            crate::error::ErrorCode::AdminNoop
        );
        self.oracle_interval_secs = new_interval;
        Ok(())
    }

    pub fn update_oracle_conf_tolerance(&mut self, new_tolerance: UFixValue64) -> Result<()> {
        let new_tolerance = validate_conf_tolerance(new_tolerance)?;
        require!(
            self.oracle_conf_tolerance != new_tolerance,
            crate::error::ErrorCode::AdminNoop
        );
        self.oracle_conf_tolerance = new_tolerance;
        Ok(())
    }

    pub fn update_mint_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        let new_fee = AssetSwapConfig::validate_fee(new_fee)?;
        require!(self.mint_fee != new_fee, crate::error::ErrorCode::AdminNoop);
        self.mint_fee = new_fee;
        Ok(())
    }

    pub fn update_redeem_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        let new_fee = AssetSwapConfig::validate_fee(new_fee)?;
        require!(
            self.redeem_fee != new_fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.redeem_fee = new_fee;
        Ok(())
    }

    pub fn update_par_tolerance(&mut self, new_tolerance: UFixValue64) -> Result<()> {
        let new_tolerance = ParTolerance::validated(new_tolerance)?;
        require!(
            self.par_tolerance != new_tolerance,
            crate::error::ErrorCode::AdminNoop
        );
        self.par_tolerance = new_tolerance;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        require!(!self.paused, crate::error::ErrorCode::AdminNoop);
        self.paused = true;
        Ok(())
    }

    pub fn unpause(&mut self) -> Result<()> {
        require!(self.paused, crate::error::ErrorCode::AdminNoop);
        self.paused = false;
        Ok(())
    }
}
