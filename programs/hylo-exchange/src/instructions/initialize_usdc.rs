use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeUsdc<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[117,115,100,99,95,112,97,105,114]}]}.
    #[account(mut)]
    pub usdc_pair: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[117,115,100,99,95,118,97,117,108,116,95,97,117,116,104]},{"kind":"account","path":"usdc_mint"}]}.
    pub usdc_vault_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[102,101,101,95,97,117,116,104]},{"kind":"account","path":"usdc_mint"}]}.
    pub usdc_fee_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"usdc_vault_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"usdc_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub usdc_collateral_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"usdc_fee_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"usdc_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub usdc_fee_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v.
    pub usdc_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub usdc_usd_pyth_feed: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA.
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL.
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=11111111111111111111111111111111.
    pub system_program: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<InitializeUsdc>,
    swap_fee: UFixValue64,
    oracle_interval_secs: u64,
    oracle_conf_tolerance: UFixValue64,
) -> Result<InitializeUsdcEvent> {
    let _ = (ctx, swap_fee, oracle_interval_secs, oracle_conf_tolerance);
    todo!()
}
