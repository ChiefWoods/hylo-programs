use crate::constants::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[allow(unused_imports)]
use crate::state::*;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializePoolDrawdownLst<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
}

pub fn handler(ctx: Context<InitializePoolDrawdownLst>) -> Result<()> {
    let mut hylo = ctx.accounts.hylo.load_mut()?;

    match hylo.pool_drawdown.outstanding() {
        Ok(_) => err!(ErrorCode::AdminNoop),
        Err(_) => {
            hylo.pool_drawdown = PoolDrawdown::default();
            Ok(())
        }
    }
}
