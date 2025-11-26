use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcBlockConfig, TransactionDetails, UiTransactionEncoding},
    client_error::ClientError,
    rpc_response::{EncodedTransaction, UiConfirmedBlock},
};

pub struct RpcSource {
    client: RpcClient,
}

impl RpcSource {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            client: RpcClient::new(rpc_url.into()),
        }
    }

    pub async fn latest_slot(
        &self,
    ) -> Result<u64, ClientError> {
        self.client.get_slot().await
    }

    pub async fn get_block(
        &self,
        slot: u64,
    ) -> Result<
        UiConfirmedBlock,
        ClientError,
    > {
        self.client
            .get_block_with_config(
                slot,
                RpcBlockConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    transaction_details: Some(TransactionDetails::Full),
                    rewards: Some(false),
                    commitment: None,
                    max_supported_transaction_version: Some(1),
                },
            )
            .await
    }
}