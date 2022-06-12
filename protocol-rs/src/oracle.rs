use ic_btc_types::Satoshi;

use crate::types::OracleErr;

/// returns btc/usd price in cents
///
/// todo:
/// 1. attach to a real oracle
pub fn get_btc_price() -> Result<u64, OracleErr> {
    Ok(2857379)
}

pub fn btc_to_satoshi(btc: u64) -> Satoshi {
    btc * 100_000_000_000
}
