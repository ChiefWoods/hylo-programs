use anchor_lang::prelude::*;
pub use fix::prelude::UFixValue64;

use crate::{
    MARINADE_STAKE_POOL_PROGRAM, SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM,
    SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM, SPL_STAKE_POOL_PROGRAM,
};

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
    pub fn address(&self) -> Pubkey {
        match self {
            Self::Spl => SPL_STAKE_POOL_PROGRAM,
            Self::SanctumSpl => SANCTUM_SPL_SOL_STAKE_POOL_PROGRAM,
            Self::SanctumSplMulti => SANCTUM_SPL_MULTI_SOL_STAKE_POOL_PROGRAM,
            Self::Marinade => MARINADE_STAKE_POOL_PROGRAM,
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
