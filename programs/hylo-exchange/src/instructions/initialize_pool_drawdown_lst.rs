use anchor_lang::prelude::*;
use crate::constants::*;

#[allow(unused_imports)]
use crate::state::*;

#[derive(Accounts)]
pub struct InitializePoolDrawdownLst<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [HYLO], bump)]
    pub hylo: Account<'info, Hylo>,
}

pub fn handler(ctx: Context<InitializePoolDrawdownLst>) -> Result<()> {
    let _ = ctx;
    todo!()
}
