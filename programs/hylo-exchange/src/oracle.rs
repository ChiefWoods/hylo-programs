use anchor_lang::prelude::*;
use hylo_core::pyth::OraclePrice;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::state::OraclePriceEvent;

pub(crate) fn load_price_update(
    account: &UncheckedAccount,
    expected_feed_id: &[u8; 32],
) -> Result<PriceUpdateV2> {
    let mut oracle_data: &[u8] = &account.try_borrow_data()?;
    let price_update = PriceUpdateV2::try_deserialize(&mut oracle_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    if price_update.price_message.feed_id != *expected_feed_id {
        return Err(ProgramError::InvalidAccountData.into());
    }
    Ok(price_update)
}

pub(crate) fn oracle_event(price: OraclePrice) -> OraclePriceEvent {
    OraclePriceEvent {
        spot: price.spot.into(),
        conf: price.conf.into(),
    }
}
