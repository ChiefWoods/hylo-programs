use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub enum AddressField {
    Admin,
    Treasury,
    PauseAuthority,
}

impl AsRef<[u8]> for AddressField {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Admin => b"admin",
            Self::Treasury => b"treasury",
            Self::PauseAuthority => b"pause_authority",
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, InitSpace)]
pub enum LstStakePoolProgram {
    Spl,
    SanctumSpl,
    SanctumSplMulti,
    Marinade,
}

impl LstStakePoolProgram {
    pub fn new(pubkey: Pubkey) -> Option<Self> {
        match pubkey {
            SPL_STAKE_POOL_PROGRAM => Some(Self::Spl),
            SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM => Some(Self::SanctumSpl),
            SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM => Some(Self::SanctumSplMulti),
            MARINADE_STAKE_POOL_PROGRAM => Some(Self::Marinade),
            _ => None,
        }
    }

    pub fn calculator(&self) -> Pubkey {
        match self {
            Self::Spl => SPL_SOL_VALUE_CALCULATOR,
            Self::SanctumSpl => SANCTUM_SPL_SOL_VALUE_CALCULATOR,
            Self::SanctumSplMulti => SANCTUM_SPL_MULTI_SOL_VALUE_CALCULATOR,
            Self::Marinade => MARINADE_SOL_VALUE_CALCULATOR,
        }
    }

    pub fn program_id(&self) -> Pubkey {
        match self {
            Self::Spl => SPL_STAKE_POOL_PROGRAM,
            Self::SanctumSpl => SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM,
            Self::SanctumSplMulti => SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM,
            Self::Marinade => MARINADE_STAKE_POOL_PROGRAM,
        }
    }

    pub fn preamble_offset(&self) -> usize {
        match self {
            Self::Spl => 0,
            Self::SanctumSpl => 4,
            Self::SanctumSplMulti => 8,
            Self::Marinade => 12,
        }
    }
}

/// Serializable oracle price for event emission.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct OraclePriceEvent {
    pub spot: UFixValue64,
    pub conf: UFixValue64,
}

/// Token metadata passed as an instruction argument.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}
