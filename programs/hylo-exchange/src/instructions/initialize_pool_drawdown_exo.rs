use crate::constants::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[allow(unused_imports)]
use crate::state::*;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializePoolDrawdownExo<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [HYLO],
        bump,
        has_one = admin,
    )]
    pub hylo: AccountLoader<'info, Hylo>,
    #[account(
        mut,
        seeds = [EXO_PAIR, collateral_mint.key().as_ref()],
        bump,
        has_one = collateral_mint,
    )]
    pub exo_pair: AccountLoader<'info, ExoPair>,
    pub collateral_mint: Account<'info, Mint>,
}

pub fn handler(ctx: Context<InitializePoolDrawdownExo>) -> Result<()> {
    let mut exo_pair = ctx.accounts.exo_pair.load_mut()?;

    match exo_pair.pool_drawdown.outstanding() {
        Ok(_) => err!(ErrorCode::AdminNoop),
        Err(_) => {
            exo_pair.pool_drawdown = PoolDrawdown::default();
            Ok(())
        }
    }
}
