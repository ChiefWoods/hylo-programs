use anchor_lang::prelude::*;
use hylo_core::{
    asset_swap_config::AssetSwapConfig,
    fees::controller::FeeController,
    pyth::{validate_conf_tolerance, validate_interval_secs, OracleConfig},
    rebalance::mode::validate_stablecoin_mint_threshold,
};

use super::*;

#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Hylo {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub lst_registry: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub pause_authority: Pubkey,
    pub stablecoin_mint_bump: u8,
    pub stablecoin_auth_bump: u8,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub registry_auth_bump: u8,
    pub total_sol_cache_bump: u8,
    pub oracle_interval_secs: u64,
    pub stablecoin_fees: StablecoinFees,
    pub levercoin_fees: LevercoinFees,
    pub total_sol_cache: TotalSolCache,
    pub yield_harvest_cache: HarvestCache,
    pub yield_harvest_config: YieldHarvestConfig,
    pub stablecoin_mint_threshold: UFixValue64,
    pub _unused_1: UFixValue64,
    pub oracle_conf_tolerance: UFixValue64,
    pub sol_usd_oracle: Pubkey,
    pub lst_swap_fee: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub lst_buy_curve_config: RebalanceCurveConfig,
    pub lst_sell_curve_config: RebalanceCurveConfig,
    pub protocol_paused: bool,
    pub lst_pair_paused: bool,
    pub _unused_2: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub _reserved: [u8; 13],
}

impl Hylo {
    pub fn get_address(&self, address_field: AddressField) -> Pubkey {
        match address_field {
            AddressField::Admin => self.admin,
            AddressField::Treasury => self.treasury,
            AddressField::PauseAuthority => self.pause_authority,
        }
    }

    pub fn set_address(&mut self, address_field: AddressField, new_address: Pubkey) -> Result<()> {
        require!(
            self.get_address(address_field.clone()) != new_address,
            crate::error::ErrorCode::AdminNoop
        );
        match address_field {
            AddressField::Admin => self.admin = new_address,
            AddressField::Treasury => self.treasury = new_address,
            AddressField::PauseAuthority => self.pause_authority = new_address,
        }
        Ok(())
    }

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

    pub fn update_sol_usd_oracle(&mut self, new_oracle: Pubkey) -> Result<()> {
        require!(
            self.sol_usd_oracle != new_oracle,
            crate::error::ErrorCode::AdminNoop
        );
        self.sol_usd_oracle = new_oracle;
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

    pub fn update_levercoin_fees(&mut self, new_fees: LevercoinFees) -> Result<()> {
        let new_fees = new_fees.validate()?;
        require!(
            self.levercoin_fees != new_fees,
            crate::error::ErrorCode::AdminNoop
        );
        self.levercoin_fees = new_fees;
        Ok(())
    }

    pub fn update_yield_harvest_config(&mut self, new_config: YieldHarvestConfig) -> Result<()> {
        let new_config = new_config
            .validate()
            .map_err(|_| error!(crate::error::ErrorCode::YieldHarvestConfigValidation))?;
        require!(
            self.yield_harvest_config.allocation != new_config.allocation
                || self.yield_harvest_config.fee != new_config.fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.yield_harvest_config = new_config;
        Ok(())
    }

    pub fn update_lst_sell_curve_config(&mut self, new_config: RebalanceCurveConfig) -> Result<()> {
        let new_config = new_config.validate_sell()?;
        require!(
            self.lst_sell_curve_config != new_config,
            crate::error::ErrorCode::AdminNoop
        );
        self.lst_sell_curve_config = new_config;
        Ok(())
    }

    pub fn update_lst_buy_curve_config(&mut self, new_config: RebalanceCurveConfig) -> Result<()> {
        let new_config = new_config.validate_buy()?;
        require!(
            self.lst_buy_curve_config != new_config,
            crate::error::ErrorCode::AdminNoop
        );
        self.lst_buy_curve_config = new_config;
        Ok(())
    }

    pub fn update_lst_swap_fee(&mut self, new_fee: UFixValue64) -> Result<()> {
        let new_fee = AssetSwapConfig::validate_fee(new_fee)?;
        require!(
            self.lst_swap_fee != new_fee,
            crate::error::ErrorCode::AdminNoop
        );
        self.lst_swap_fee = new_fee;
        Ok(())
    }

    pub fn refresh_lst_vault(
        &mut self,
        price_sol: &LstSolPrice,
        amount_before: fix::prelude::UFix64<fix::prelude::N9>,
        amount_after: fix::prelude::UFix64<fix::prelude::N9>,
        current_epoch: u64,
    ) -> Result<()> {
        let sol_before = price_sol.convert_lst_to_sol(amount_before, current_epoch)?;
        let sol_after = price_sol.convert_lst_to_sol(amount_after, current_epoch)?;
        self.total_sol_cache.decrement(sol_before, current_epoch)?;
        self.total_sol_cache.increment(sol_after, current_epoch)?;
        Ok(())
    }

    pub fn pause_protocol(&mut self) -> Result<()> {
        require!(!self.protocol_paused, crate::error::ErrorCode::AdminNoop);
        self.protocol_paused = true;
        Ok(())
    }

    pub fn unpause_protocol(&mut self) -> Result<()> {
        require!(self.protocol_paused, crate::error::ErrorCode::AdminNoop);
        self.protocol_paused = false;
        Ok(())
    }

    pub fn pause_lst_pair(&mut self) -> Result<()> {
        require!(!self.lst_pair_paused, crate::error::ErrorCode::AdminNoop);
        self.lst_pair_paused = true;
        Ok(())
    }

    pub fn unpause_lst_pair(&mut self) -> Result<()> {
        require!(self.lst_pair_paused, crate::error::ErrorCode::AdminNoop);
        self.lst_pair_paused = false;
        Ok(())
    }
}
