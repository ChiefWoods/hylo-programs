use anchor_lang::prelude::*;

// not included in IDL
pub const MINT_AUTH: &[u8] = b"mint_auth";

#[constant]
pub const POOL_AUTH: &[u8] = b"pool_auth";

#[constant]
pub const POOL_CONFIG: &[u8] = b"pool_config";

#[constant]
pub const STAKED_HYUSD: &[u8] = b"staked_hyUSD";

pub const METAPLEX_TOKEN_METADATA: Pubkey = Pubkey::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");