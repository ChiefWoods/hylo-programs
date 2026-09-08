use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::{events::*, state::*};

#[derive(Accounts)]
pub struct RegisterLst<'info> {
    /// CHECK: IDL metadata: writable; signer; relations=hylo.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[104,121,108,111]}]}.
    pub hylo: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"const","value":[108,115,116,95,104,101,97,100,101,114]},{"kind":"account","path":"lst_mint"}]}.
    #[account(mut)]
    pub lst_header: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[102,101,101,95,97,117,116,104]},{"kind":"account","path":"lst_mint"}]}.
    pub fee_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[118,97,117,108,116,95,97,117,116,104]},{"kind":"account","path":"lst_mint"}]}.
    pub vault_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: pda={"seeds":[{"kind":"const","value":[108,115,116,95,114,101,103,105,115,116,114,121,95,97,117,116,104]}]}.
    pub registry_auth: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"fee_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"lst_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; pda={"seeds":[{"kind":"account","path":"vault_auth"},{"kind":"const","value":[6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]},{"kind":"account","path":"lst_mint"}],"program":{"kind":"const","value":[140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]}}.
    #[account(mut)]
    pub lst_vault: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub lst_mint: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: writable; relations=hylo.
    #[account(mut)]
    pub lst_registry: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub lst_stake_pool_state: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sanctum_calculator_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub sanctum_calculator_state: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub stake_pool_program_data: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: no additional constraints.
    pub stake_pool_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=AddressLookupTab1e1111111111111111111111111.
    pub lut_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL.
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA.
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: IDL metadata: address=11111111111111111111111111111111.
    pub system_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<RegisterLst>, rebalance_fee: UFixValue64) -> Result<RegisterLstEvent> {
    let _ = (ctx, rebalance_fee);
    todo!()
}
