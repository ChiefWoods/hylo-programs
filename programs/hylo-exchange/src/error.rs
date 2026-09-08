use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Cannot redeem levercoin due to Depeg. NAV would be 0 or lower.")]
    LevercoinRedeemDisabled,
    #[msg("Levercoin to stablecoin swap disabled due to rebalance mode.")]
    LeverToStableDisabled,
    #[msg("Stablecoin to levercoin swap disabled due to rebalance mode.")]
    StableToLeverDisabled,
    #[msg("Error during CPI to Sanctum LST/SOL calculator.")]
    SanctumCpi,
    #[msg("LST registry cannot be initialized twice.")]
    LstRegistryAlreadyInitialized,
    #[msg("LST registry calculators were already added to lookup table.")]
    LstRegistryCalculatorsAlreadyInitialized,
    #[msg("LST specific accounts found empty when attempting to load registry.")]
    LstRegistryEmpty,
    #[msg("Sanctum calculator contexts in LST registry preamble are malformed.")]
    LstRegistryPreamble,
    #[msg("Failed to deserialize registry lookup table.")]
    LstRegistryLookupTableDeser,
    #[msg("Contents of LST registry did not match remaining_accounts.")]
    LstRegistryLookupTableInvalid,
    #[msg("Mint/vault/pool accounts in registry block do not match header.")]
    LstBlockInvalid,
    #[msg("Attempted to register an LST with invalid Sanctum context accounts.")]
    LstContextInvalid,
    #[msg("Addition overflow while computing total SOL in LST registry.")]
    LstAdditionOverflow,
    #[msg("Cached LST price not from current epoch. Run pricing crank to update.")]
    LstPriceOutdated,
    #[msg("Failed to compute delta between current and previous LST prices.")]
    LstPriceDelta,
    #[msg("Found current epoch less than previous in LST header.")]
    LstPriceEpochsInvalid,
    #[msg("Overflow while computing LST SOL appreciation.")]
    LstSolAppreciation,
    #[msg("Stablecoin mint disabled. Collateral ratio is below minting threshold.")]
    StablecoinMintDisabled,
    #[msg("Levercoin mint disabled Due to depeg.")]
    LevercoinMintDisabled,
    #[msg("Yield harvest configuration percentages failed validation.")]
    YieldHarvestConfigValidation,
    #[msg("Yield harvest has already occurred during this epoch.")]
    YieldHarvestAlreadyRun,
    #[msg("Arithmetic error while computing yield harvest allocation.")]
    YieldHarvestAllocation,
    #[msg("Yield harvest already occurred during this epoch.")]
    YieldHarvestEpoch,
    #[msg("Cannot swap from an asset to itself.")]
    IdentitySwap,
    #[msg("Incorrect decimals assumption for given collateral mint.")]
    ExoAmountDecimals,
    #[msg("Unable to upconvert amount to desired exponent.")]
    ExoAmountUpConversion,
    #[msg("Oracle for exo collateral does not match given feed_id.")]
    ExoOracleInvalid,
    #[msg("Borrow rate harvest has already occurred during this epoch.")]
    BorrowRateHarvestAlreadyRun,
    #[msg("Underflow while computing elapsed epochs for borrow rate harvest.")]
    BorrowRateHarvestEpochUnderflow,
    #[msg("Virtual stablecoin already initialized for LSTs.")]
    LstVirtualStablecoinAlreadyInitialized,
    #[msg("Cannot update configuration with identical value.")]
    AdminNoop,
    #[msg("Sell-side rebalancing is inactive at current collateral ratio.")]
    RebalanceSellInactive,
    #[msg("Buy-side rebalancing is inactive at current collateral ratio.")]
    RebalanceBuyInactive,
    #[msg("Mint is not in the exogenous collateral allowlist.")]
    ExoMintNotInAllowlist,
    #[msg("Address change proposal has expired.")]
    AddressChangeExpired,
    #[msg("Address change proposal's TTL not in configured range.")]
    AddressChangeTtlInvalid,
    #[msg("Address change proposal has not been approved.")]
    AddressChangeNotApproved,
    #[msg("Address change proposal has already been approved.")]
    AddressChangeAlreadyApproved,
    #[msg("Address change approver must be upgrade authority.")]
    AddressChangeUpgradeAuthority,
    #[msg("Failed while converting precision for a token amount.")]
    TokenAmountPrecisionError,
    #[msg("Underflow while computing virtual stablecoin delta.")]
    SettleVirtualStablecoinUnderflow,
    #[msg("Underflow while converting TVL to stablecoin.")]
    SettleVirtualStablecoinConversion,
    #[msg("Virtual stablecoin settlement noop: nothing to drawdown or repay.")]
    SettleVirtualStablecoinNoop,
    #[msg("Rebalance PnL settlement disabled in Depeg mode.")]
    SettleRebalancePnlDisabled,
    #[msg("LST stake pool is not supported.")]
    LstStakePoolNotSupported,
    #[msg("Exo pair genesis mint constraints not met.")]
    ExoGenesisConstraints,
    #[msg("Error or constraint not met for collateral ratio for genesis.")]
    ExoGenesisCollateralRatio,
    #[msg("Cannot unpause exo pair due to zero virtual stablecoin.")]
    ExoPairZeroVirtualStablecoin,
}
