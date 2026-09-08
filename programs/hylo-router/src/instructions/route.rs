use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct Route {}

pub fn handler(
    ctx: Context<Route>,
    token_a: Pubkey,
    token_b: Pubkey,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<()> {
    let _ = (ctx, token_a, token_b, amount, slippage_config);
    todo!()
}
