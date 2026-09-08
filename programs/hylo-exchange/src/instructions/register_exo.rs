use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct RegisterExo<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[101,120,111,95,112,97,105,114]},{"kind":"account","path":"collateral_mint"}]}.
    #[account(mut)]
    pub exo_pair: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[109,105,110,116,95,97,117,116,104]},{"kind":"account","path":"levercoin_mint"}]}.
    pub levercoin_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[101,120,111,95,118,97,117,108,116,95,97,117,116,104]},{"kind":"account","path":"collateral_mint"}]}.
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[102,101,101,95,97,117,116,104]},{"kind":"account","path":"collateral_mint"}]}.
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"vault_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"collateral_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub collateral_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"fee_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"collateral_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub collateral_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[101,120,111,95,108,101,118,101,114,99,111,105,110]},{"kind":"account","path":"collateral_mint"}]}.
    #[account(mut)]
    pub levercoin_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable.
    #[account(mut)]
    pub levercoin_metadata: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub exo_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s.
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: IDL metadata: address=ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL.
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=SysvarRent111111111111111111111111111111111.
    pub rent: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterExo>,
    oracle_feed_id: [u8; 32],
    oracle_interval_secs: u64,
    oracle_conf_tolerance: UFixValue64,
    stablecoin_mint_threshold: UFixValue64,
    borrow_rate_config: BorrowRateConfig,
    levercoin_fees: LevercoinFees,
    sell_curve_config: RebalanceCurveConfig,
    buy_curve_config: RebalanceCurveConfig,
    metadata: TokenMetadata,
    levercoin_market_cap_limit: UFixValue64,
) -> Result<RegisterExoEvent> {
    let _ = (
        ctx,
        oracle_feed_id,
        oracle_interval_secs,
        oracle_conf_tolerance,
        stablecoin_mint_threshold,
        borrow_rate_config,
        levercoin_fees,
        sell_curve_config,
        buy_curve_config,
        metadata,
        levercoin_market_cap_limit,
    );
    todo!()
}
