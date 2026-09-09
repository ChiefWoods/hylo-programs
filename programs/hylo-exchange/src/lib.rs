pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod lst_registry;
pub(crate) mod oracle;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("HYEXCHtHkBagdStcJCp3xbbb9B7sdMdWXFNj6mdsG4hn");
declare_program!(hylo_earn_pool);

#[program]
pub mod hylo_exchange {
    use super::*;

    pub fn accept_address_update(
        ctx: Context<AcceptAddressUpdate>,
        address_field: AddressField,
    ) -> Result<AcceptAddressUpdateEvent> {
        accept_address_update::handler(ctx, address_field)
    }

    pub fn approve_address_update(
        ctx: Context<ApproveAddressUpdate>,
        address_field: AddressField,
    ) -> Result<ApproveAddressUpdateEvent> {
        approve_address_update::handler(ctx, address_field)
    }

    pub fn cancel_address_update(
        ctx: Context<CancelAddressUpdate>,
        address_field: AddressField,
    ) -> Result<CancelAddressUpdateEvent> {
        cancel_address_update::handler(ctx, address_field)
    }

    pub fn convert_lever_to_stable_exo(
        ctx: Context<ConvertLeverToStableExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<ConvertLeverToStableExoEvent> {
        convert_lever_to_stable_exo::handler(ctx, amount, slippage_config)
    }

    pub fn convert_lever_to_stable_lst(
        ctx: Context<ConvertLeverToStableLst>,
        amount_levercoin: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<ConvertLeverToStableLstEvent> {
        convert_lever_to_stable_lst::handler(ctx, amount_levercoin, slippage_config)
    }

    pub fn convert_stable_to_lever_exo(
        ctx: Context<ConvertStableToLeverExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<ConvertStableToLeverExoEvent> {
        convert_stable_to_lever_exo::handler(ctx, amount, slippage_config)
    }

    pub fn convert_stable_to_lever_lst(
        ctx: Context<ConvertStableToLeverLst>,
        amount_stablecoin: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<ConvertStableToLeverLstEvent> {
        convert_stable_to_lever_lst::handler(ctx, amount_stablecoin, slippage_config)
    }

    pub fn genesis_mint_exo(
        ctx: Context<GenesisMintExo>,
        amount: u64,
    ) -> Result<GenesisMintExoEvent> {
        genesis_mint_exo::handler(ctx, amount)
    }

    pub fn harvest_borrow_rate(ctx: Context<HarvestBorrowRate>) -> Result<HarvestBorrowRateEvent> {
        harvest_borrow_rate::handler(ctx)
    }

    pub fn harvest_yield(ctx: Context<HarvestYield>) -> Result<HarvestYieldEvent> {
        harvest_yield::handler(ctx)
    }

    pub fn initialize_lst_registry(ctx: Context<InitializeLstRegistry>, slot: u64) -> Result<()> {
        initialize_lst_registry::handler(ctx, slot)
    }

    pub fn initialize_lst_registry_calculators(
        ctx: Context<InitializeLstRegistryCalculators>,
    ) -> Result<()> {
        initialize_lst_registry_calculators::handler(ctx)
    }

    pub fn initialize_lst_virtual_stablecoin(
        ctx: Context<InitializeLstVirtualStablecoin>,
    ) -> Result<InitializeLstVirtualStablecoinEvent> {
        initialize_lst_virtual_stablecoin::handler(ctx)
    }

    pub fn initialize_mints(
        ctx: Context<InitializeMints>,
        stablecoin_metadata: TokenMetadata,
        levercoin_metadata: TokenMetadata,
    ) -> Result<()> {
        initialize_mints::handler(ctx, stablecoin_metadata, levercoin_metadata)
    }

    pub fn initialize_pool_drawdown_exo(ctx: Context<InitializePoolDrawdownExo>) -> Result<()> {
        initialize_pool_drawdown_exo::handler(ctx)
    }

    pub fn initialize_pool_drawdown_lst(ctx: Context<InitializePoolDrawdownLst>) -> Result<()> {
        initialize_pool_drawdown_lst::handler(ctx)
    }

    pub fn initialize_protocol(
        ctx: Context<InitializeProtocol>,
        pause_authority: Pubkey,
        oracle_interval_secs: u64,
        stablecoin_mint_threshold: UFixValue64,
        levercoin_fees: LevercoinFees,
        yield_harvest_config: YieldHarvestConfig,
    ) -> Result<()> {
        initialize_protocol::handler(
            ctx,
            pause_authority,
            oracle_interval_secs,
            stablecoin_mint_threshold,
            levercoin_fees,
            yield_harvest_config,
        )
    }

    pub fn initialize_usdc(
        ctx: Context<InitializeUsdc>,
        mint_fee: UFixValue64,
        redeem_fee: UFixValue64,
        oracle_interval_secs: u64,
        oracle_conf_tolerance: UFixValue64,
        par_tolerance: UFixValue64,
    ) -> Result<InitializeUsdcEvent> {
        initialize_usdc::handler(
            ctx,
            mint_fee,
            redeem_fee,
            oracle_interval_secs,
            oracle_conf_tolerance,
            par_tolerance,
        )
    }

    pub fn mint_levercoin_exo(
        ctx: Context<MintLevercoinExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<MintLevercoinExoEvent> {
        mint_levercoin_exo::handler(ctx, amount, slippage_config)
    }

    pub fn mint_levercoin_lst(
        ctx: Context<MintLevercoinLst>,
        amount_lst_to_deposit: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<MintLevercoinLstEvent> {
        mint_levercoin_lst::handler(ctx, amount_lst_to_deposit, slippage_config)
    }

    pub fn mint_stablecoin_exo(
        ctx: Context<MintStablecoinExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<MintStablecoinExoEvent> {
        mint_stablecoin_exo::handler(ctx, amount, slippage_config)
    }

    pub fn mint_stablecoin_lst(
        ctx: Context<MintStablecoinLst>,
        amount_lst_to_deposit: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<MintStablecoinLstEvent> {
        mint_stablecoin_lst::handler(ctx, amount_lst_to_deposit, slippage_config)
    }

    pub fn mint_stablecoin_usdc(
        ctx: Context<MintStablecoinUsdc>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<MintStablecoinUsdcEvent> {
        mint_stablecoin_usdc::handler(ctx, amount, slippage_config)
    }

    pub fn pause_exo_pair(ctx: Context<PauseExoPair>) -> Result<PauseEvent> {
        pause_exo_pair::handler(ctx)
    }

    pub fn pause_lst_pair(ctx: Context<PauseLstPair>) -> Result<PauseEvent> {
        pause_lst_pair::handler(ctx)
    }

    pub fn pause_protocol(ctx: Context<PauseProtocol>) -> Result<PauseEvent> {
        pause_protocol::handler(ctx)
    }

    pub fn pause_usdc_pair(ctx: Context<PauseUsdcPair>) -> Result<PauseEvent> {
        pause_usdc_pair::handler(ctx)
    }

    pub fn propose_address_update(
        ctx: Context<ProposeAddressUpdate>,
        address_field: AddressField,
        ttl_secs: u64,
    ) -> Result<ProposeAddressUpdateEvent> {
        propose_address_update::handler(ctx, address_field, ttl_secs)
    }

    pub fn redeem_levercoin_exo(
        ctx: Context<RedeemLevercoinExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<RedeemLevercoinExoEvent> {
        redeem_levercoin_exo::handler(ctx, amount, slippage_config)
    }

    pub fn redeem_levercoin_lst(
        ctx: Context<RedeemLevercoinLst>,
        amount_to_redeem: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<RedeemLevercoinLstEvent> {
        redeem_levercoin_lst::handler(ctx, amount_to_redeem, slippage_config)
    }

    pub fn redeem_stablecoin_exo(
        ctx: Context<RedeemStablecoinExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<RedeemStablecoinExoEvent> {
        redeem_stablecoin_exo::handler(ctx, amount, slippage_config)
    }

    pub fn redeem_stablecoin_lst(
        ctx: Context<RedeemStablecoinLst>,
        amount_to_redeem: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<RedeemStablecoinLstEvent> {
        redeem_stablecoin_lst::handler(ctx, amount_to_redeem, slippage_config)
    }

    pub fn redeem_stablecoin_usdc(
        ctx: Context<RedeemStablecoinUsdc>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<RedeemStablecoinUsdcEvent> {
        redeem_stablecoin_usdc::handler(ctx, amount, slippage_config)
    }

    pub fn register_exo(
        ctx: Context<RegisterExo>,
        oracle_feed_id: [u8; 32],
        oracle_interval_secs: u64,
        oracle_conf_tolerance: UFixValue64,
        stablecoin_mint_threshold: UFixValue64,
        borrow_rate_curve_config: BorrowRateCurveConfig,
        borrow_rate_fee: UFixValue64,
        levercoin_fees: LevercoinFees,
        sell_curve_config: RebalanceCurveConfig,
        buy_curve_config: RebalanceCurveConfig,
        metadata: TokenMetadata,
        levercoin_market_cap_limit: UFixValue64,
    ) -> Result<RegisterExoEvent> {
        register_exo::handler(
            ctx,
            oracle_feed_id,
            oracle_interval_secs,
            oracle_conf_tolerance,
            stablecoin_mint_threshold,
            borrow_rate_curve_config,
            borrow_rate_fee,
            levercoin_fees,
            sell_curve_config,
            buy_curve_config,
            metadata,
            levercoin_market_cap_limit,
        )
    }

    pub fn register_lst(
        ctx: Context<RegisterLst>,
        rebalance_fee: UFixValue64,
    ) -> Result<RegisterLstEvent> {
        register_lst::handler(ctx, rebalance_fee)
    }

    pub fn settle_virtual_stablecoin_exo(
        ctx: Context<SettleVirtualStablecoinExo>,
    ) -> Result<SettleVirtualStablecoinExoEvent> {
        settle_virtual_stablecoin_exo::handler(ctx)
    }

    pub fn settle_virtual_stablecoin_lst(
        ctx: Context<SettleVirtualStablecoinLst>,
    ) -> Result<SettleVirtualStablecoinLstEvent> {
        settle_virtual_stablecoin_lst::handler(ctx)
    }

    pub fn settle_virtual_stablecoin_usdc(
        ctx: Context<SettleVirtualStablecoinUsdc>,
    ) -> Result<SettleVirtualStablecoinUsdcEvent> {
        settle_virtual_stablecoin_usdc::handler(ctx)
    }

    pub fn swap_exo_to_usdc(
        ctx: Context<SwapExoToUsdc>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_exo_to_usdc::handler(ctx, amount, slippage_config)
    }

    pub fn swap_exo_to_usdc_all(
        ctx: Context<SwapExoToUsdcAll>,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_exo_to_usdc_all::handler(ctx, slippage_config)
    }

    pub fn swap_lst_to_lst(
        ctx: Context<SwapLstToLst>,
        amount_lst_a: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<SwapLstToLstEvent> {
        swap_lst_to_lst::handler(ctx, amount_lst_a, slippage_config)
    }

    pub fn swap_lst_to_usdc(
        ctx: Context<SwapLstToUsdc>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_lst_to_usdc::handler(ctx, amount, slippage_config)
    }

    pub fn swap_lst_to_usdc_all(
        ctx: Context<SwapLstToUsdcAll>,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_lst_to_usdc_all::handler(ctx, slippage_config)
    }

    pub fn swap_usdc_to_exo(
        ctx: Context<SwapUsdcToExo>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_usdc_to_exo::handler(ctx, amount, slippage_config)
    }

    pub fn swap_usdc_to_lst(
        ctx: Context<SwapUsdcToLst>,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        swap_usdc_to_lst::handler(ctx, amount, slippage_config)
    }

    pub fn unpause_exo_pair(ctx: Context<UnpauseExoPair>) -> Result<UnpauseEvent> {
        unpause_exo_pair::handler(ctx)
    }

    pub fn unpause_lst_pair(ctx: Context<UnpauseLstPair>) -> Result<UnpauseEvent> {
        unpause_lst_pair::handler(ctx)
    }

    pub fn unpause_protocol(ctx: Context<UnpauseProtocol>) -> Result<UnpauseEvent> {
        unpause_protocol::handler(ctx)
    }

    pub fn unpause_usdc_pair(ctx: Context<UnpauseUsdcPair>) -> Result<UnpauseEvent> {
        unpause_usdc_pair::handler(ctx)
    }

    pub fn update_exo_borrow_rate_curve(
        ctx: Context<UpdateExoBorrowRateCurve>,
        new_curve_config: BorrowRateCurveConfig,
    ) -> Result<UpdateBorrowRateCurveConfigEvent> {
        update_exo_borrow_rate_curve::handler(ctx, new_curve_config)
    }

    pub fn update_exo_borrow_rate_fee(
        ctx: Context<UpdateExoBorrowRateFee>,
        new_borrow_rate_fee: UFixValue64,
    ) -> Result<UpdateFeeEvent> {
        update_exo_borrow_rate_fee::handler(ctx, new_borrow_rate_fee)
    }

    pub fn update_exo_buy_curve(
        ctx: Context<UpdateExoBuyCurve>,
        new_buy_curve_config: RebalanceCurveConfig,
    ) -> Result<UpdateRebalanceCurveConfigEvent> {
        update_exo_buy_curve::handler(ctx, new_buy_curve_config)
    }

    pub fn update_exo_levercoin_fees(
        ctx: Context<UpdateExoLevercoinFees>,
        new_levercoin_fees: LevercoinFees,
    ) -> Result<UpdateLevercoinFeesEvent> {
        update_exo_levercoin_fees::handler(ctx, new_levercoin_fees)
    }

    pub fn update_exo_levercoin_market_cap_limit(
        ctx: Context<UpdateExoLevercoinMarketCapLimit>,
        new_levercoin_market_cap_limit: UFixValue64,
    ) -> Result<UpdateLevercoinMarketCapLimitEvent> {
        update_exo_levercoin_market_cap_limit::handler(ctx, new_levercoin_market_cap_limit)
    }

    pub fn update_exo_oracle(
        ctx: Context<UpdateExoOracle>,
        new_oracle: Pubkey,
    ) -> Result<UpdateOracleAddressEvent> {
        update_exo_oracle::handler(ctx, new_oracle)
    }

    pub fn update_exo_oracle_conf_tolerance(
        ctx: Context<UpdateExoOracleConfTolerance>,
        new_oracle_conf_tolerance: UFixValue64,
    ) -> Result<UpdateOracleConfEvent> {
        update_exo_oracle_conf_tolerance::handler(ctx, new_oracle_conf_tolerance)
    }

    pub fn update_exo_oracle_interval(
        ctx: Context<UpdateExoOracleInterval>,
        new_oracle_interval_secs: u64,
    ) -> Result<UpdateOracleIntervalEvent> {
        update_exo_oracle_interval::handler(ctx, new_oracle_interval_secs)
    }

    pub fn update_exo_sell_curve(
        ctx: Context<UpdateExoSellCurve>,
        new_sell_curve_config: RebalanceCurveConfig,
    ) -> Result<UpdateRebalanceCurveConfigEvent> {
        update_exo_sell_curve::handler(ctx, new_sell_curve_config)
    }

    pub fn update_exo_stablecoin_mint_threshold(
        ctx: Context<UpdateExoStablecoinMintThreshold>,
        new_stablecoin_mint_threshold: UFixValue64,
    ) -> Result<UpdateStablecoinMintThresholdEvent> {
        update_exo_stablecoin_mint_threshold::handler(ctx, new_stablecoin_mint_threshold)
    }

    pub fn update_levercoin_fees(
        ctx: Context<UpdateLevercoinFees>,
        new_levercoin_fees: LevercoinFees,
    ) -> Result<UpdateLevercoinFeesEvent> {
        update_levercoin_fees::handler(ctx, new_levercoin_fees)
    }

    pub fn update_lst_buy_curve_config(
        ctx: Context<UpdateLstBuyCurveConfig>,
        new_buy_curve_config: RebalanceCurveConfig,
    ) -> Result<UpdateRebalanceCurveConfigEvent> {
        update_lst_buy_curve_config::handler(ctx, new_buy_curve_config)
    }

    pub fn update_lst_prices(ctx: Context<UpdateLstPrices>) -> Result<UpdateLstPricesEvent> {
        update_lst_prices::handler(ctx)
    }

    pub fn update_lst_rebalance_fee(
        ctx: Context<UpdateLstRebalanceFee>,
        new_rebalance_fee: UFixValue64,
    ) -> Result<UpdateLstRebalanceFeeEvent> {
        update_lst_rebalance_fee::handler(ctx, new_rebalance_fee)
    }

    pub fn update_lst_sell_curve_config(
        ctx: Context<UpdateLstSellCurveConfig>,
        new_sell_curve_config: RebalanceCurveConfig,
    ) -> Result<UpdateRebalanceCurveConfigEvent> {
        update_lst_sell_curve_config::handler(ctx, new_sell_curve_config)
    }

    pub fn update_lst_stablecoin_mint_threshold(
        ctx: Context<UpdateLstStablecoinMintThreshold>,
        new_stablecoin_mint_threshold: UFixValue64,
    ) -> Result<UpdateStablecoinMintThresholdEvent> {
        update_lst_stablecoin_mint_threshold::handler(ctx, new_stablecoin_mint_threshold)
    }

    pub fn update_lst_swap_fee(
        ctx: Context<UpdateLstSwapFee>,
        new_lst_swap_fee: UFixValue64,
    ) -> Result<UpdateFeeEvent> {
        update_lst_swap_fee::handler(ctx, new_lst_swap_fee)
    }

    pub fn update_oracle_conf_tolerance(
        ctx: Context<UpdateOracleConfTolerance>,
        new_oracle_conf_tolerance: UFixValue64,
    ) -> Result<UpdateOracleConfEvent> {
        update_oracle_conf_tolerance::handler(ctx, new_oracle_conf_tolerance)
    }

    pub fn update_oracle_interval(
        ctx: Context<UpdateOracleInterval>,
        new_oracle_interval_secs: u64,
    ) -> Result<UpdateOracleIntervalEvent> {
        update_oracle_interval::handler(ctx, new_oracle_interval_secs)
    }

    pub fn update_sol_usd_oracle(
        ctx: Context<UpdateSolUsdOracle>,
        new_oracle: Pubkey,
    ) -> Result<UpdateOracleAddressEvent> {
        update_sol_usd_oracle::handler(ctx, new_oracle)
    }

    pub fn update_usdc_oracle_conf_tolerance(
        ctx: Context<UpdateUsdcOracleConfTolerance>,
        new_oracle_conf_tolerance: UFixValue64,
    ) -> Result<UpdateOracleConfEvent> {
        update_usdc_oracle_conf_tolerance::handler(ctx, new_oracle_conf_tolerance)
    }

    pub fn update_usdc_oracle_interval(
        ctx: Context<UpdateUsdcOracleInterval>,
        new_oracle_interval_secs: u64,
    ) -> Result<UpdateOracleIntervalEvent> {
        update_usdc_oracle_interval::handler(ctx, new_oracle_interval_secs)
    }

    pub fn update_par_tolerance(
        ctx: Context<UpdateParTolerance>,
        new_par_tolerance: UFixValue64,
    ) -> Result<UpdateParToleranceEvent> {
        update_par_tolerance::handler(ctx, new_par_tolerance)
    }

    pub fn update_usdc_mint_fee(
        ctx: Context<UpdateUsdcMintFee>,
        new_mint_fee: UFixValue64,
    ) -> Result<UpdateFeeEvent> {
        update_usdc_mint_fee::handler(ctx, new_mint_fee)
    }

    pub fn update_usdc_redeem_fee(
        ctx: Context<UpdateUsdcRedeemFee>,
        new_redeem_fee: UFixValue64,
    ) -> Result<UpdateFeeEvent> {
        update_usdc_redeem_fee::handler(ctx, new_redeem_fee)
    }

    pub fn update_yield_harvest_config(
        ctx: Context<UpdateYieldHarvestConfig>,
        new_yield_harvest_config: YieldHarvestConfig,
    ) -> Result<UpdateYieldHarvestConfigEvent> {
        update_yield_harvest_config::handler(ctx, new_yield_harvest_config)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<WithdrawFeesEvent> {
        withdraw_fees::handler(ctx)
    }
}
