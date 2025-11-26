use sflowana::config::Config;

fn main() {
    println!("sflowana {}", sflowana::VERSION);

    dotenvy::dotenv().ok();

    let config = Config::from_env()
        .expect("failed to load configuration");

    println!("RPC: {}", config.rpc_url);
}