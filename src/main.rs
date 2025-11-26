use sflowana::config::Config;
use sflowana::solana::rpc::RpcSource;

#[tokio::main]
async fn main() {
    println!("sflowana {}", sflowana::VERSION);

    dotenvy::dotenv().ok();

    let config = Config::from_env()
        .expect("failed to load configuration");

    println!("RPC: {}", config.rpc_url);

    let rpc = RpcSource::new(config.rpc_url);

    let slot = rpc
        .latest_slot()
        .await
        .expect("failed to fetch latest slot");

    println!("latest slot: {slot}");
}