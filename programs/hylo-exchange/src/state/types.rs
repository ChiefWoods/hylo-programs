use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct AcceptAddressUpdateEvent {
    pub address_field: AddressField,
    pub old_address: Pubkey,
    pub new_address: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AddressField {
    Admin,
    Treasury,
    PauseAuthority,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApproveAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}

/// Per-epoch borrow rate for exogenous collateral without native yield.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct BorrowRateConfig {
    pub rate: UFixValue64,
    pub fee: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CancelAddressUpdateEvent {
    pub address_field: AddressField,
    pub new_address: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
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

/// Swaps
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConvertLeverToStableLstEvent {
    pub levercoin_burned: UFixValue64,
    pub levercoin_nav: UFixValue64,
    pub stablecoin_minted_user: UFixValue64,
    pub stablecoin_minted_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConvertStableToLeverLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub stablecoin_nav: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub levercoin_nav: UFixValue64,
}

/// Represents the spread of fees between mint and redeem for protocol tokens.
/// All fees must be in basis points to represent a fractional percentage
/// directly applicable to a token amount e.g. `0.XXXX` or `bips x 10^-4`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FeePair {
    pub mint: UFixValue64,
    pub redeem: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenesisMintExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub levercoin_minted: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub collateral_ratio: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct HarvestBorrowRateEvent {
    pub collateral_mint: Pubkey,
    pub levercoin_market_cap: UFixValue64,
    pub total_stablecoin_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
}

/// Records epoch harvest information for off-chain consumers.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct HarvestCache {
    pub epoch: u64,
    pub stability_pool_cap: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
}

/// Cranks
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct HarvestYieldEvent {
    pub total_sol_harvested: UFixValue64,
    pub fees_extracted: UFixValue64,
    pub token_to_pool: UFixValue64,
    pub pool_drawdown_repaid: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeLstVirtualStablecoinEvent {
    pub stablecoin_amount: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeUsdcEvent {
    pub vault_auth_bump: u8,
    pub fee_auth_bump: u8,
    pub swap_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct LevercoinFees {
    pub normal: FeePair,
    pub sell_zone_1: FeePair,
    pub sell_zone_2: FeePair,
}

/// Captures the true LST price in SOL for the current epoch.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct LstSolPrice {
    pub price: UFixValue64,
    pub epoch: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum LstStakePoolProgram {
    Spl,
    SanctumSpl,
    SanctumSplMulti,
    Marinade,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MintLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub oracle: Pubkey,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MintLevercoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
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

/// Mint / Redeem
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MintStablecoinLstEvent {
    pub minted: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_deposited: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MintStablecoinUsdcEvent {
    pub usdc_deposited: UFixValue64,
    pub usdc_fees: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}

/// Serializable oracle price for event emission.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OraclePriceEvent {
    pub spot: UFixValue64,
    pub conf: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PauseEvent {}

/// Outstanding hyUSD debt owed to the earn pool after a Depeg absorption.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PoolDrawdown {
    pub ledger: VirtualStablecoin,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct PriceFeedMessage {
    /// `FeedId` but avoid the type alias because of compatibility issues with Anchor's `idl-build` feature.
    pub feed_id: [u8; 32],
    pub price: i64,
    pub conf: u64,
    pub exponent: i32,
    /// The timestamp of this price update in seconds
    pub publish_time: i64,
    /// The timestamp of the previous price update. This field is intended to allow users to
    /// identify the single unique price update for any moment in time:
    /// for any time t, the unique update is the one such that prev_publish_time < t <= publish_time.
    ///
    /// Note that there may not be such an update while we are migrating to the new message-sending logic,
    /// as some price updates on pythnet may not be sent to other chains (because the message-sending
    /// logic may not have triggered). We can solve this problem by making the message-sending mandatory
    /// (which we can do once publishers have migrated over).
    ///
    /// Additionally, this field may be equal to publish_time if the message is sent on a slot where
    /// where the aggregation was unsuccesful. This problem will go away once all publishers have
    /// migrated over to a recent version of pyth-agent.
    pub prev_publish_time: i64,
    pub ema_price: i64,
    pub ema_conf: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProposeAddressUpdateEvent {
    pub address_field: AddressField,
    pub current_address: Pubkey,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
}

/// Floor/ceil deviation percentages for rebalance price curve construction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RebalanceCurveConfig {
    pub floor_pct: UFixValue64,
    pub ceil_pct: UFixValue64,
}

/// Serializable version of [`RebalancePnl`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum RebalancePnlValue {
    Profit(UFixValue64),
    Loss(UFixValue64),
    NoChange,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RedeemLevercoinExoEvent {
    pub collateral_mint: Pubkey,
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub collateral_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RedeemLevercoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RedeemStablecoinLstEvent {
    pub redeemed: UFixValue64,
    pub nav: UFixValue64,
    pub sol_usd_price: OraclePriceEvent,
    pub lst_mint: Pubkey,
    pub lst_sol_price: UFixValue64,
    pub collateral_withdrawn: UFixValue64,
    pub fees_deposited: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RedeemStablecoinUsdcEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_fees: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub virtual_stablecoin_supply: UFixValue64,
}

/// V2
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RegisterExoEvent {
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub oracle: Pubkey,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub borrow_rate: UFixValue64,
    pub borrow_rate_fee: UFixValue64,
}

/// Admin
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RegisterLstEvent {
    pub header: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub pool_state: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SettleRebalancePnlExoEvent {
    pub collateral_mint: Pubkey,
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SettleRebalancePnlLstEvent {
    pub pnl: RebalancePnlValue,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SettleVirtualStablecoinExoEvent {
    pub collateral_mint: Pubkey,
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SettleVirtualStablecoinLstEvent {
    pub stablecoin_burned: UFixValue64,
    pub stablecoin_minted: UFixValue64,
    pub virtual_stablecoin_supply: UFixValue64,
    pub pool_drawdown_outstanding: UFixValue64,
    pub pool_balance: UFixValue64,
}

/// Client specified slippage tolerance paired with expected token amount.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SlippageConfig {
    pub expected_token_out: UFixValue64,
    pub slippage_tolerance: UFixValue64,
}

/// **Deprecated** — retained only for `Hylo` account deserialization.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct StablecoinFees {
    pub normal: FeePair,
    pub mode_1: FeePair,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapExoToUsdcEvent {
    pub collateral_mint: Pubkey,
    pub collateral_deposited: UFixValue64,
    pub collateral_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapLstToLstEvent {
    pub lst_a_mint: Pubkey,
    pub lst_a_in: UFixValue64,
    pub lst_a_fees_extracted: UFixValue64,
    pub lst_b_mint: Pubkey,
    pub lst_b_out: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapLstToUsdcEvent {
    pub lst_mint: Pubkey,
    pub lst_deposited: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
    pub usdc_withdrawn: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapUsdcToExoEvent {
    pub collateral_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub collateral_withdrawn: UFixValue64,
    pub collateral_usd_price: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapUsdcToLstEvent {
    pub lst_mint: Pubkey,
    pub usdc_deposited: UFixValue64,
    pub usdc_usd_price: OraclePriceEvent,
    pub lst_withdrawn: UFixValue64,
    pub sol_rebalance_usd_price: UFixValue64,
}

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TotalSolCache {
    pub current_update_epoch: u64,
    pub total_sol: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UnpauseEvent {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateExoBorrowRateEvent {
    pub old_borrow_rate_config: BorrowRateConfig,
    pub new_borrow_rate_config: BorrowRateConfig,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateLevercoinFeesEvent {
    pub old_levercoin_fees: LevercoinFees,
    pub new_levercoin_fees: LevercoinFees,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateLevercoinMarketCapLimitEvent {
    pub old_levercoin_market_cap_limit: UFixValue64,
    pub new_levercoin_market_cap_limit: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateLstPricesEvent {
    pub updated_mints: Vec<Pubkey>,
    pub new_total_sol: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateLstRebalanceFeeEvent {
    pub lst_mint: Pubkey,
    pub old_rebalance_fee: UFixValue64,
    pub new_rebalance_fee: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateOracleAddressEvent {
    pub old_oracle: Pubkey,
    pub new_oracle: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateOracleConfEvent {
    pub old_oracle_conf_tolerance: UFixValue64,
    pub new_oracle_conf_tolerance: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateOracleIntervalEvent {
    pub old_oracle_interval_secs: u64,
    pub new_oracle_interval_secs: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateRebalanceCurveConfigEvent {
    pub old_curve_config: RebalanceCurveConfig,
    pub new_curve_config: RebalanceCurveConfig,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateStablecoinMintThresholdEvent {
    pub old_stablecoin_mint_threshold: UFixValue64,
    pub new_stablecoin_mint_threshold: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateSwapFeeEvent {
    pub old_swap_fee: UFixValue64,
    pub new_swap_fee: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateYieldHarvestConfigEvent {
    pub old_yield_harvest_config: YieldHarvestConfig,
    pub new_yield_harvest_config: YieldHarvestConfig,
}

/// Pyth price updates are bridged to all blockchains via Wormhole.
/// Using the price updates on another chain requires verifying the signatures of the Wormhole guardians.
/// The usual process is to check the signatures for two thirds of the total number of guardians, but this can be cumbersome on Solana because of the transaction size limits,
/// so we also allow for partial verification.
///
/// This enum represents how much a price update has been verified:
/// - If `Full`, we have verified the signatures for two thirds of the current guardians.
/// - If `Partial`, only `num_signatures` guardian signatures have been checked.
///
/// # Warning
/// Using partially verified price updates is dangerous, as it lowers the threshold of guardians that need to collude to produce a malicious price update.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum VerificationLevel {
    Partial { num_signatures: u8 },
    Full,
}

/// Simple counter representing the supply of a "virtual" stablecoin.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct VirtualStablecoin {
    pub supply: UFixValue64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct WithdrawFeesEvent {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub treasury_ata: Pubkey,
    pub amount: u64,
}

/// Captures yield harvest configuration as two basis point values:
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct YieldHarvestConfig {
    pub allocation: UFixValue64,
    pub fee: UFixValue64,
}
