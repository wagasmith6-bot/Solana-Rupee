use crate::damm_api;

pub async fn sniper_mainnet() {
    let top = damm_api::fetch_top_20().await.unwrap();

    for p in top["data"].as_array().unwrap() {
        let liquidity = p["liquidity"].as_f64().unwrap_or(0.0);
        if liquidity > 50000.0 {
            println!("High liquidity pool: {:?}", p["address"]);
            // prepare swap
        }
    }
}