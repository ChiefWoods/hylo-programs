pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("hyRouTRDAgn65xyyJ3L5c4k5SFmSdr3NxDV8Euzjy3f");
declare_program!(hylo_exchange);
declare_program!(hylo_earn_pool);

#[program]
pub mod hylo_router {
    use super::*;

    pub fn route(
        ctx: Context<Route>,
        token_a: Pubkey,
        token_b: Pubkey,
        amount: u64,
        slippage_config: Option<SlippageConfig>,
    ) -> Result<()> {
        route::handler(ctx, token_a, token_b, amount, slippage_config)
    }
}
