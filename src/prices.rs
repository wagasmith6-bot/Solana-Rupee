use std::env;

pub async fn get_price(token_id: &str) -> f64 {
    let url = format!("{}?ids={}&vs_currencies=usd", env::var("PRICE_API").unwrap(), token_id);
    let res: serde_json::Value = reqwest::get(&url).await.unwrap().json().await.unwrap();
    res[token_id]["usd"].as_f64().unwrap()
}