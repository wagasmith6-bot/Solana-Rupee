use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::damm_api;

pub async fn scan_and_trade() {

    let pools = match damm_api::fetch_top_20().await {
        Ok(v) => v,
        Err(_) => return
    };

    let list = pools["data"].as_array().unwrap_or(&vec![]);

    for pool in list {

        let liquidity =
            pool["liquidity"].as_f64().unwrap_or(0.0);

        let price_impact_threshold = dec!(0.05);

        if liquidity > 50000.0 {

            println!("High probability pool detected");

            // Here you build swap + jito bundle
        }
    }
}