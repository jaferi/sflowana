use sflowana::config::Config;
use sflowana::solana::block::Block;
use sflowana::solana::rpc::RpcSource;

fn rpc_source() -> RpcSource {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("SFLOWANA_RPC_URL must be set");

    RpcSource::new(config.rpc_url)
}

#[tokio::test]
async fn fetches_latest_slot() {
    let rpc = rpc_source();

    let slot = rpc
        .latest_slot()
        .await
        .expect("failed to fetch latest slot");

    assert!(slot > 0);
}

#[tokio::test]
async fn fetches_block() {
    let rpc = rpc_source();

    let slot = rpc
        .latest_slot()
        .await
        .expect("failed to fetch latest slot");

    let block = rpc.get_block(slot).await.expect("failed to fetch block");

    assert!(block.blockhash.is_ascii());
}

#[tokio::test]
async fn extracts_transactions() {
    let rpc = rpc_source();

    let slot = rpc
        .latest_slot()
        .await
        .expect("failed to fetch latest slot");

    let block = rpc.get_block(slot).await.expect("failed to fetch block");

    let block: Block = Block::from_ui_confirmed_block(slot, &block);

    assert!(!block.transactions.is_empty());
    assert!(!block.transactions[0].signature.is_empty());
}
