use anchor_lang::prelude::*;
use hylo_core::{
    borrow_rate::validate_borrow_rate_fee,
    fees::controller::FeeController,
    limiter::levercoin::validate_levercoin_market_cap_limit,
    pyth::{validate_conf_tolerance, validate_interval_secs, OracleConfig},
    rebalance::mode::validate_stablecoin_mint_threshold,
};

use super::*;

#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct ExoPair {
    pub collateral_mint: Pubkey,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub oracle: Pubkey,
    pub oracle_feed_id: [u8; 32],
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub stablecoin_mint_threshold: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub borrow_rate_curve_config: BorrowRateCurveConfig,
    pub borrow_rate_harvest_cache: HarvestCache,
    pub levercoin_fees: LevercoinFees,
    pub sell_curve_config: RebalanceCurveConfig,
    pub buy_curve_config: RebalanceCurveConfig,
    pub borrow_rate_fee: UFixValue64,
    pub paused: bool,
    pub levercoin_market_cap_limit: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub virtual_stablecoin_supply_floor: UFixValue64,
    pub _reserved: [u8; 91],
}

impl ExoPair {
    pub fn oracle_config(&self) -> Result<OracleConfig> {
        Ok(OracleConfig::new(
            self.oracle_interval_secs,
            self.oracle_conf_tolerance.try_into()?,
        ))
    }

    pub fn swap_oracle_config(&self) -> Result<OracleConfig> {
        self.oracle_config()
    }

    pub fn stablecoin_mint_threshold(&self) -> Result<fix::prelude::UFix64<fix::prelude::N9>> {
        Ok(self.stablecoin_mint_threshold.try_into()?)
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

    pub fn update_oracle(&mut self, new_oracle: Pubkey) -> Result<()> {
        require!(
            self.oracle != new_oracle,
            crate::error::ErrorCode::AdminNoop
        );
        self.oracle = new_oracle;
        Ok(())
    }

    pub fn update_stablecoin_mint_threshold(&mut self, new_threshold: UFixValue64) -> Result<()> {
        let new_threshold = validate_stablecoin_mint_threshold(new_threshold)?;
        require!(
            self.stablecoin_mint_threshold != new_threshold,
            crate::error::ErrorCode::AdminNoop
        );
        self.stablecoin_mint_threshold = new_threshold;
        Ok(())
    }

    pub fn update_borrow_rate_curve(&mut self, new_config: BorrowRateCurveConfig) -> Result<()> {
        let new_config = new_config.validate()?;
        require!(
            self.borrow_rate_curve_config != new_config,
            crate::error::ErrorCode::AdminNoop
        );
        self.borrow_rate_curve_config = new_config;
        Ok(())
    }

    pub fn update_borrow_rate_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        let new_fee = validate_borrow_rate_fee(new_fee)?;
        require!(
            self.borrow_rate_fee != new_fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.borrow_rate_fee = new_fee;
        Ok(())
    }

    pub fn update_levercoin_fees(&mut self, new_fees: LevercoinFees) -> Result<()> {
        let new_fees = new_fees.validate()?;
        require!(
            self.levercoin_fees != new_fees,
            crate::error::ErrorCode::AdminNoop
        );
        self.levercoin_fees = new_fees;
        Ok(())
    }

    pub fn update_sell_curve(&mut self, new_config: RebalanceCurveConfig) -> Result<()> {
        let new_config = new_config.validate_sell()?;
        require!(
            self.sell_curve_config != new_config,
            crate::error::ErrorCode::AdminNoop
        );
        self.sell_curve_config = new_config;
        Ok(())
    }

    pub fn update_buy_curve(&mut self, new_config: RebalanceCurveConfig) -> Result<()> {
        let new_config = new_config.validate_buy()?;
        require!(
            self.buy_curve_config != new_config,
            crate::error::ErrorCode::AdminNoop
        );
        self.buy_curve_config = new_config;
        Ok(())
    }

    pub fn update_levercoin_market_cap_limit(&mut self, new_limit: UFixValue64) -> Result<()> {
        let new_limit = validate_levercoin_market_cap_limit(new_limit)?;
        require!(
            self.levercoin_market_cap_limit != new_limit,
            crate::error::ErrorCode::AdminNoop
        );
        self.levercoin_market_cap_limit = new_limit;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        require!(!self.paused, crate::error::ErrorCode::AdminNoop);
        self.paused = true;
        Ok(())
    }

    pub fn unpause(&mut self) -> Result<()> {
        require!(self.paused, crate::error::ErrorCode::AdminNoop);
        require!(
            self.virtual_stablecoin.supply()? > fix::prelude::UFix64::zero(),
            crate::error::ErrorCode::ExoPairZeroVirtualStablecoin
        );
        self.paused = false;
        Ok(())
    }
}
