use anchor_lang::prelude::*;
use fix::prelude::UFixValue64;

#[constant]
pub const ADDRESS_UPDATE_PROPOSAL: &[u8] = b"address_update_proposal";

#[constant]
pub const DEAD: &[u8] = b"dead";

#[constant]
pub const EVENT_AUTH: &[u8] = b"__event_authority";

#[constant]
pub const EXO_LEVERCOIN: &[u8] = b"exo_levercoin";

#[constant]
pub const EXO_PAIR: &[u8] = b"exo_pair";

#[constant]
pub const EXO_VAULT_AUTH: &[u8] = b"exo_vault_auth";

#[constant]
pub const FEE_AUTH: &[u8] = b"fee_auth";

#[constant]
pub const HYLO: &[u8] = b"hylo";

#[constant]
pub const HYUSD: &[u8] = b"hyUSD";

#[constant]
pub const LST_HEADER: &[u8] = b"lst_header";

#[constant]
pub const LST_REGISTRY_AUTH: &[u8] = b"lst_registry_auth";

#[constant]
pub const MINT_AUTH: &[u8] = b"mint_auth";

#[constant]
pub const POOL_AUTH: &[u8] = b"pool_auth";

#[constant]
pub const SETTLEMENT_AUTH: &[u8] = b"settlement_auth";

#[constant]
pub const USDC_PAIR: &[u8] = b"usdc_pair";

#[constant]
pub const USDC_VAULT_AUTH: &[u8] = b"usdc_vault_auth";

#[constant]
pub const VAULT_AUTH: &[u8] = b"vault_auth";

#[constant]
pub const XSOL: &[u8] = b"xSOL";

pub const HYLO_EARN_POOL: Pubkey = crate::hylo_earn_pool::ID;

pub const METAPLEX_TOKEN_METADATA: Pubkey = anchor_spl::metadata::ID;

pub const SPL_STAKE_POOL_PROGRAM: Pubkey =
    Pubkey::from_str_const("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");

// single validator
pub const SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM: Pubkey =
    Pubkey::from_str_const("SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY");

// multi-validator
pub const SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM: Pubkey =
    Pubkey::from_str_const("SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn");

pub const MARINADE_STAKE_POOL_PROGRAM: Pubkey =
    Pubkey::from_str_const("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

pub const SPL_SOL_VALUE_CALCULATOR: Pubkey =
    Pubkey::from_str_const("sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF");

pub const SANCTUM_SPL_SOL_VALUE_CALCULATOR: Pubkey =
    Pubkey::from_str_const("sspUE1vrh7xRoXxGsg7vR1zde2WdGtJRbyK9uRumBDy");

pub const SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR: Pubkey =
    Pubkey::from_str_const("ssmbu3KZxgonUtjEMCKspZzxvUQCxAFnyh1rcHUeEDo");

pub const MARINADE_SOL_VALUE_CALCULATOR: Pubkey =
    Pubkey::from_str_const("mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP");

/// Canonical calculator state accounts, recovered from the deployed LST registry ALT.
pub const SPL_SOL_VALUE_CALCULATOR_STATE: Pubkey =
    Pubkey::from_str_const("7orJ4kDhn1Ewp54j29tBzUWDFGhyimhYi7sxybZcphHd");

pub const SANCTUM_SPL_SOL_VALUE_CALCULATOR_STATE: Pubkey =
    Pubkey::from_str_const("84C2M1NcmqFiP37qHKzuz8ydyyjCrzNqY77GhvHtCpyf");

pub const SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR_STATE: Pubkey =
    Pubkey::from_str_const("Ehcuy2BzuY9BscqcH2K43tDKqoi6xQHxChtVjzrMfvU8");

pub const MARINADE_SOL_VALUE_CALCULATOR_STATE: Pubkey =
    Pubkey::from_str_const("FMbUjYFtqgm4Zfpg7MguUg33RQ3tvkd22NgaCCAs3M6E");

pub const SPL_STAKE_POOL_PROGRAM_DATA: Pubkey =
    Pubkey::from_str_const("EmiU8AQkB2sswTxVB6aCmsAJftoowZGGDXuytm6X65R3");

pub const SANCTUM_SPL_STAKE_POOL_PROGRAM_DATA: Pubkey =
    Pubkey::from_str_const("Cn5fegqLh8Fmvffisr4Wk3LmuaUgMMzTFfEuidpZFsvV");

pub const SANCTUM_SPL_MULTI_STAKE_POOL_PROGRAM_DATA: Pubkey =
    Pubkey::from_str_const("HxBTMuB7cFBPVWVJjTi9iBF8MPd7mfY1QnrrWfLAySFd");

pub const MARINADE_STAKE_POOL_PROGRAM_DATA: Pubkey =
    Pubkey::from_str_const("4PQH9YmfuKrVyZaibkLYpJZPv2FPaybhq2GAuBcWMSBf");

/// Deployed SOL/USD oracle confidence tolerance: 1% N9.
pub const DEFAULT_ORACLE_CONF_TOLERANCE: UFixValue64 = UFixValue64 {
    bits: 10_000_000,
    exp: -9,
};

/// Neutral-zone midpoint used as the exogenous genesis collateral ratio.
pub const GENESIS_TARGET_COLLATERAL_RATIO: u64 = 1_500_000_000;

pub const LST_DECIMALS: u8 = 9;

pub const PROTOCOL_TOKEN_DECIMALS: u8 = 6;

pub const STABLECOIN_TOKEN_NAME: &str = "Hylo USD";

pub const LST_LEVERCOIN_TOKEN_NAME: &str = "Hylo 3x Leveraged SOL";

/// Canonical calculator preamble: (calculator, calculator state, stake program, program data) × 4 variants.
pub const LST_REGISTRY_CALCULATOR_PREAMBLE_LEN: usize = 16;

/// Per-LST registry block: header, mint, vault, pool state.
pub const LST_REGISTRY_BLOCK_LEN: usize = 4;
