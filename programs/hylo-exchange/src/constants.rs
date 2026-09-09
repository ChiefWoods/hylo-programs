use anchor_lang::prelude::*;

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

// not imported from hylo-earn-pool due to circular dependency
pub const HYLO_EARN_POOL: Pubkey = Pubkey::from_str_const("HysTabVUfmQBFcmzu1ctRd1Y1fxd66RBpboy1bmtDSQQ");

pub const METAPLEX_TOKEN_METADATA: Pubkey = Pubkey::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");