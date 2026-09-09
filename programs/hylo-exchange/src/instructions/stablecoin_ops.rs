use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, TransferChecked};
use fix::prelude::{UFix64, N6};

use crate::constants::*;
use crate::hylo_earn_pool;
use crate::state::PoolDrawdown;

pub(crate) fn transfer_user<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    token::transfer_checked(
        CpiContext::new(
            token_program,
            TransferChecked {
                from,
                mint,
                to,
                authority,
            },
        ),
        amount,
        decimals,
    )
}

pub(crate) fn transfer_pda<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    token::transfer_checked(
        CpiContext::new_with_signer(
            token_program,
            TransferChecked {
                from,
                mint,
                to,
                authority,
            },
            &[signer_seeds],
        ),
        amount,
        decimals,
    )
}

pub(crate) fn burn_tokens<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    token::burn(
        CpiContext::new(
            token_program,
            Burn {
                mint,
                from,
                authority,
            },
        ),
        amount,
    )
}

pub(crate) fn mint_stablecoin<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint_key: Pubkey,
    auth_bump: u8,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let bump = [auth_bump];
    let seeds: &[&[u8]] = &[MINT_AUTH, mint_key.as_ref(), &bump];
    token::mint_to(
        CpiContext::new_with_signer(
            token_program,
            MintTo {
                mint,
                to,
                authority,
            },
            &[seeds],
        ),
        amount,
    )
}

pub(crate) fn absorb_loss<'info>(
    earn_pool: AccountInfo<'info>,
    settlement_auth: AccountInfo<'info>,
    hylo: AccountInfo<'info>,
    pool_config: AccountInfo<'info>,
    pool_auth: AccountInfo<'info>,
    stablecoin_pool: AccountInfo<'info>,
    stablecoin_mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    settlement_auth_bump: u8,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let bump = [settlement_auth_bump];
    let seeds: &[&[u8]] = &[SETTLEMENT_AUTH, &bump];
    hylo_earn_pool::cpi::absorb_loss(
        CpiContext::new_with_signer(
            earn_pool,
            hylo_earn_pool::cpi::accounts::AbsorbLoss {
                settlement_auth,
                hylo,
                pool_config,
                pool_auth,
                stablecoin_pool,
                stablecoin_mint,
                token_program,
            },
            &[seeds],
        ),
        amount,
    )?;
    Ok(())
}

pub(crate) fn drawdown_repay(
    pool_drawdown: &mut PoolDrawdown,
    pool_proceeds: UFix64<N6>,
) -> Result<UFix64<N6>> {
    let repay = pool_proceeds.min(pool_drawdown.outstanding()?);
    if repay > UFix64::zero() {
        pool_drawdown.repay(repay)?;
    }
    Ok(repay)
}
