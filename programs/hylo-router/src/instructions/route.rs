use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::{get_return_data, invoke, set_return_data};
use anchor_lang::InstructionData;

use crate::constants::{HYUSD, SHYUSD, USDC, XSOL};
use crate::error::ErrorCode;
use crate::hylo_earn_pool;
use crate::hylo_exchange;
use crate::state::SlippageConfig;

#[derive(Accounts)]
pub struct Route {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Stable,
    LstLever,
    EarnLp,
    Usdc,
    Lst,
    ExoCollateral,
    ExoLever,
}

pub fn handler(
    ctx: Context<Route>,
    token_a: Pubkey,
    token_b: Pubkey,
    amount: u64,
    slippage_config: Option<SlippageConfig>,
) -> Result<()> {
    require!(token_a != token_b, ErrorCode::UnsupportedPair);

    let remaining = ctx.remaining_accounts;
    let kind_a = classify(token_a, remaining)?;
    let kind_b = classify(token_b, remaining)?;

    match (kind_a, kind_b) {
        (TokenKind::Lst, TokenKind::Stable) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::MintStablecoinLst {
                amount_lst_to_deposit: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::Lst) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::RedeemStablecoinLst {
                amount_to_redeem: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Lst, TokenKind::LstLever) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::MintLevercoinLst {
                amount_lst_to_deposit: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::LstLever, TokenKind::Lst) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::RedeemLevercoinLst {
                amount_to_redeem: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::LstLever) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::ConvertStableToLeverLst {
                amount_stablecoin: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::LstLever, TokenKind::Stable) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::ConvertLeverToStableLst {
                amount_levercoin: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Lst, TokenKind::Lst) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::SwapLstToLst {
                amount_lst_a: amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Usdc, TokenKind::Stable) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::MintStablecoinUsdc {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::Usdc) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::RedeemStablecoinUsdc {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Lst, TokenKind::Usdc) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::SwapLstToUsdc {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Usdc, TokenKind::Lst) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::SwapUsdcToLst {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::EarnLp) => cpi_earn_pool(
            remaining,
            hylo_earn_pool::client::args::UserDeposit {
                amount_stablecoin: amount,
                slippage_config: map_earn(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::EarnLp, TokenKind::Stable) => cpi_earn_pool(
            remaining,
            hylo_earn_pool::client::args::UserWithdraw {
                amount_lp_token: amount,
                slippage_config: map_earn(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::ExoCollateral, TokenKind::Stable) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::MintStablecoinExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::ExoCollateral) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::RedeemStablecoinExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::ExoCollateral, TokenKind::ExoLever) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::MintLevercoinExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::ExoLever, TokenKind::ExoCollateral) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::RedeemLevercoinExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Stable, TokenKind::ExoLever) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::ConvertStableToLeverExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::ExoLever, TokenKind::Stable) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::ConvertLeverToStableExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::ExoCollateral, TokenKind::Usdc) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::SwapExoToUsdc {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        (TokenKind::Usdc, TokenKind::ExoCollateral) => cpi_exchange(
            remaining,
            hylo_exchange::client::args::SwapUsdcToExo {
                amount,
                slippage_config: map_ex(&slippage_config)?,
            }
            .data(),
        ),
        _ => err!(ErrorCode::UnsupportedPair),
    }
}

fn classify<'info>(mint: Pubkey, remaining: &[AccountInfo<'info>]) -> Result<TokenKind> {
    match mint {
        HYUSD => return Ok(TokenKind::Stable),
        XSOL => return Ok(TokenKind::LstLever),
        SHYUSD => return Ok(TokenKind::EarnLp),
        USDC => return Ok(TokenKind::Usdc),
        _ => {}
    }

    let lst_header = pda(
        &hylo_exchange::ID,
        &[hylo_exchange::constants::LST_HEADER.as_ref(), mint.as_ref()],
    );
    if remaining.iter().any(|acc| acc.key() == lst_header) {
        return Ok(TokenKind::Lst);
    }

    let exo_pair = pda(
        &hylo_exchange::ID,
        &[hylo_exchange::constants::EXO_PAIR.as_ref(), mint.as_ref()],
    );
    if remaining.iter().any(|acc| acc.key() == exo_pair) {
        return Ok(TokenKind::ExoCollateral);
    }

    if is_exo_levercoin(remaining, mint)? {
        return Ok(TokenKind::ExoLever);
    }

    err!(ErrorCode::UnsupportedPair)
}

fn is_exo_levercoin<'info>(remaining: &[AccountInfo<'info>], lever: Pubkey) -> Result<bool> {
    for acc in remaining {
        let data = acc.try_borrow_data()?;
        if data.len() < 40 || !data.starts_with(hylo_exchange::accounts::ExoPair::DISCRIMINATOR) {
            continue;
        }
        let mut collateral_bytes = [0u8; 32];
        collateral_bytes.copy_from_slice(&data[8..40]);
        drop(data);
        let collateral = Pubkey::from(collateral_bytes);
        let derived_lever = pda(
            &hylo_exchange::ID,
            &[
                hylo_exchange::constants::EXO_LEVERCOIN.as_ref(),
                collateral.as_ref(),
            ],
        );
        if derived_lever == lever {
            return Ok(true);
        }
    }
    Ok(false)
}

fn cpi_exchange<'info>(remaining: &[AccountInfo<'info>], data: Vec<u8>) -> Result<()> {
    cpi(hylo_exchange::ID, remaining, data)
}

fn cpi_earn_pool<'info>(remaining: &[AccountInfo<'info>], data: Vec<u8>) -> Result<()> {
    cpi(hylo_earn_pool::ID, remaining, data)
}

fn cpi<'info>(program_id: Pubkey, remaining: &[AccountInfo<'info>], data: Vec<u8>) -> Result<()> {
    let accounts = remaining
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();
    invoke(
        &Instruction {
            program_id,
            accounts,
            data,
        },
        remaining,
    )?;
    if let Some((_, data)) = get_return_data() {
        set_return_data(&data);
    }
    Ok(())
}

fn pda(program_id: &Pubkey, seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, program_id).0
}

fn map_ex(
    slippage_config: &Option<SlippageConfig>,
) -> Result<Option<hylo_exchange::types::SlippageConfig>> {
    recast(slippage_config)
}

fn map_earn(
    slippage_config: &Option<SlippageConfig>,
) -> Result<Option<hylo_earn_pool::types::SlippageConfig>> {
    recast(slippage_config)
}

fn recast<T, U>(value: &T) -> Result<U>
where
    T: AnchorSerialize,
    U: AnchorDeserialize,
{
    let bytes = value
        .try_to_vec()
        .map_err(|_| error!(ErrorCode::InvalidRouteAccounts))?;
    U::try_from_slice(&bytes).map_err(|_| error!(ErrorCode::InvalidRouteAccounts))
}
