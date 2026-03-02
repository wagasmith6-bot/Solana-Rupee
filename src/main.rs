mod damm_api;
mod mev_engine;
mod monitor;
mod jito;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("MEV Bot Running...");

    tokio::spawn(async {
        monitor::monitor_new_pools().await;
    });

    loop {
        mev_engine::scan_and_trade().await;
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}