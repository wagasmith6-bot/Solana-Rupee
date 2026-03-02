use std::env;

pub async fn send_bundle(base64_tx: String) {
    let endpoint = env::var("JITO_ENDPOINT").unwrap();
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "transactions":[base64_tx],
        "tip": 50000 // tip higher for real competition on mainnet
    });

    let _ = client.post(endpoint).json(&body).send().await;
}