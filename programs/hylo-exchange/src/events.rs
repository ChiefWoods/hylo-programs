use crate::state::*;
use anchor_lang::prelude::*;

#[event]
#[derive(Clone)]
pub struct AcceptAddressUpdateEvent {
    pub address_field: AddressField,
    pub old_address: Pubkey,
    pub new_address: Pubkey,
}

#[event]
#[derive(Clone)]
pub struct ApproveAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}

#[event]
#[derive(Clone)]
pub struct CancelAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}

#[event]
pub struct ConvertLeverToStableExoEvent {
    pub collateral_mint: Pubkey,
    pub levercoin_burned: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub stablecoin_minted_user: UFixValue64,
    pub stablecoin_minted_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}

#[event]
pub struct ConvertLeverToStableLstEvent {
    pub levercoin_burned: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub stablecoin_minted_user: UFixValue64,
    pub stablecoin_minted_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
}

#[event]
pub struct ConvertStableToLeverExoEvent {
    pub collateral_mint: Pubkey,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}

#[event]
pub struct ConvertStableToLeverLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub levercoin_nav: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct GenesisMintExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub collateral_ratio: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}

#[event]
pub struct HarvestBorrowRateEvent {
    pub collateral_mint: Pubkey,
    pub levercoin_market_cap: UFixValue64,
    pub total_stablecoin_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}

#[event]
pub struct HarvestYieldEvent {
    pub total_sol_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub token_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
}

#[event]
#[derive(Clone)]
pub struct InitializeLstVirtualStablecoinEvent {
    pub stablecoin_amount: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct InitializeUsdcEvent {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub mint_fee: UFixValue64,
    pub redeem_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub par_tolerance: UFixValue64,
}

#[event]
pub struct MintLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub oracle: Pubkey,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
pub struct MintLevercoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
pub struct MintStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub stablecoin_supply: UFixValue64,
}

#[event]
pub struct MintStablecoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct MintStablecoinUsdcEvent {
    pub usdc_deposited: UFixValue64,
    pub usdc_fees: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
}

#[event]
pub struct PauseEvent {}

#[event]
#[derive(Clone)]
pub struct ProposeAddressUpdateEvent {
    pub address_field: AddressField,
    pub current_address: Pubkey,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
}

#[event]
pub struct RedeemLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
pub struct RedeemLevercoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
pub struct RedeemStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub stablecoin_supply: UFixValue64,
}

#[event]
pub struct RedeemStablecoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct RedeemStablecoinUsdcEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct RegisterExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub oracle: Pubkey,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub borrow_rate_curve_config: BorrowRateCurveConfig,
    pub borrow_rate_fee: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct RegisterLstEvent {
    pub header: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub pool_state: Pubkey,
}

#[event]
#[derive(Clone)]
pub struct SettleRebalancePnlExoEvent {
    pub collateral_mint: Pubkey,
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct SettleRebalancePnlLstEvent {
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}

#[event]
pub struct SettleVirtualStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}

#[event]
pub struct SettleVirtualStablecoinLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}

#[event]
pub struct SettleVirtualStablecoinUsdcEvent {
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_balance: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct SwapExoToUsdcEvent {
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub collateral_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
}

#[event]
pub struct SwapLstToLstEvent {
    pub lst_a_mint: Pubkey,
    pub lst_a_in: UFixValue64,
    pub lst_a_fees_extracted: UFixValue64,
    pub lst_b_mint: Pubkey,
    pub lst_b_out: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct SwapLstToUsdcEvent {
    pub lst_mint: Pubkey,
    pub lst_deposited: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
}

#[event]
#[derive(Clone)]
pub struct SwapUsdcToExoEvent {
    pub collateral_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub collateral_usd_price: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct SwapUsdcToLstEvent {
    pub lst_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub lst_withdrawn: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
}

#[event]
pub struct UnpauseEvent {}

#[event]
#[derive(Clone)]
pub struct UpdateBorrowRateCurveConfigEvent {
    pub old_curve_config: BorrowRateCurveConfig,
    pub new_curve_config: BorrowRateCurveConfig,
}

#[event]
#[derive(Clone)]
pub struct UpdateLevercoinFeesEvent {
    pub old_levercoin_fees: LevercoinFees,
    pub new_levercoin_fees: LevercoinFees,
}

#[event]
#[derive(Clone)]
pub struct UpdateLevercoinMarketCapLimitEvent {
    pub old_levercoin_market_cap_limit: UFixValue64,
    pub new_levercoin_market_cap_limit: UFixValue64,
}

#[event]
pub struct UpdateLstPricesEvent {
    pub updated_mints: Vec<Pubkey>,
    pub new_total_sol: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateLstRebalanceFeeEvent {
    pub lst_mint: Pubkey,
    pub old_rebalance_fee: UFixValue64,
    pub new_rebalance_fee: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateOracleAddressEvent {
    pub old_oracle: Pubkey,
    pub new_oracle: Pubkey,
}

#[event]
#[derive(Clone)]
pub struct UpdateOracleConfEvent {
    pub old_oracle_conf_tolerance: UFixValue64,
    pub new_oracle_conf_tolerance: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateOracleIntervalEvent {
    pub old_oracle_interval_secs: u64,
    pub new_oracle_interval_secs: u64,
}

#[event]
#[derive(Clone)]
pub struct UpdateRebalanceCurveConfigEvent {
    pub old_curve_config: RebalanceCurveConfig,
    pub new_curve_config: RebalanceCurveConfig,
}

#[event]
#[derive(Clone)]
pub struct UpdateStablecoinMintThresholdEvent {
    pub old_stablecoin_mint_threshold: UFixValue64,
    pub new_stablecoin_mint_threshold: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateFeeEvent {
    pub old_fee: UFixValue64,
    pub new_fee: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateParToleranceEvent {
    pub old_par_tolerance: UFixValue64,
    pub new_par_tolerance: UFixValue64,
}

#[event]
#[derive(Clone)]
pub struct UpdateYieldHarvestConfigEvent {
    pub old_yield_harvest_config: YieldHarvestConfig,
    pub new_yield_harvest_config: YieldHarvestConfig,
}

#[event]
#[derive(Clone)]
pub struct WithdrawFeesEvent {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub treasury_ata: Pubkey,
    pub amount: u64,
}
