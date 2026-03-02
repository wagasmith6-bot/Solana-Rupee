use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn send_bundle(tx: String) {

    let endpoint =
        env::var("JITO_ENDPOINT")
        .unwrap_or("https://mainnet.block-engine.jito.wtf/api/v1/bundles".to_string());

    let client = Client::new();

    let body = json!({
        "transactions":[tx],
        "tip": 50000
    });

    let _ = client.post(endpoint)
        .json(&body)
        .send()
        .await;
}