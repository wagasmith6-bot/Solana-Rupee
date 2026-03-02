use reqwest::Client;
use serde_json::Value;
use std::env;

pub async fn fetch_all_pools() -> Result<Value, reqwest::Error> {
    let base = env::var("DAMM_API").unwrap();
    let url = format!("{}/pools", base);
    let client = Client::new();
    let res = client.get(url).send().await?;
    Ok(res.json().await?)
}

pub async fn fetch_top_20() -> Result<Value, reqwest::Error> {
    let base = env::var("DAMM_API").unwrap();
    let url = format!("{}/pools?limit=20&sortBy=liquidity", base);
    let client = Client::new();
    let res = client.get(url).send().await?;
    Ok(res.json().await?)
}

// If mainnet API requires headers, uncomment and use:
// pub async fn fetch_top_20_with_headers() -> Result<Value, reqwest::Error> {
//     let base = env::var("DAMM_API").unwrap();
//     let url = format!("{}/pools?limit=20&sortBy=liquidity", base);
//     let client = Client::new();
//     let res = client
//         .get(url)
//         .header("x-api-key", env::var("DAMM_API_KEY").unwrap())
//         .send()
//         .await?;
//     Ok(res.json().await?)
// }