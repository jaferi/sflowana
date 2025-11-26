use solana_client::nonblocking::rpc_client::RpcClient;

pub struct RpcSource {
    client: RpcClient,
}

impl RpcSource {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            client: RpcClient::new(rpc_url.into()),
        }
    }

    pub async fn latest_slot(&self) -> Result<u64, solana_client::client_error::ClientError> {
        self.client.get_slot().await
    }
}