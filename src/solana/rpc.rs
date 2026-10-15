use std::{future::Future, time::Duration};
use tokio::time::sleep;

use solana_client::{
    client_error::ClientError,
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcBlockConfig, TransactionDetails, UiTransactionEncoding},
    rpc_response::UiConfirmedBlock,
};

const MAX_ATTEMPTS: u32 = 10;
const INITIAL_DELAY: Duration = Duration::from_millis(250);
const MAX_DELAY: Duration = Duration::from_secs(10);

// generic retry function that takes an async operation and retries it with exponential backoff
async fn retry<T, F, Fut>(mut operation: F) -> Result<T, ClientError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, ClientError>>,
{
    let mut delay = INITIAL_DELAY;

    for attempt in 1..=MAX_ATTEMPTS {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) if attempt == MAX_ATTEMPTS => return Err(error),
            Err(error) if !error_is_retryable(&error) => return Err(error),
            Err(error) => {
                eprintln!(
                    "RPC attempt {attempt}/{MAX_ATTEMPTS} failed: {error}. Retrying in {delay:?}"
                );

                sleep(delay).await;

                // random jitter to avoid thundering herd problem
                let jitter = u64::from(attempt) * 100;
                let noise = u64::from(std::process::id()) % (jitter + 1);

                delay = (delay * 2).min(MAX_DELAY) + Duration::from_millis(noise);
            }
        }
    }

    unreachable!()
}

// check if the error is retryable based on its message or status code.
fn error_is_retryable(error: &ClientError) -> bool {
    let message = error.to_string().to_lowercase();

    message.contains("429")
        || message.contains("too many requests")
        || message.contains("rate limit")
        || message.contains("timed out")
        || message.contains("timeout")
        || message.contains("connection reset")
        || message.contains("connection refused")
        || message.contains("temporarily unavailable")
        || message.contains("502")
        || message.contains("503")
        || message.contains("504")
}

pub struct RpcSource {
    client: RpcClient,
}

impl RpcSource {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            client: RpcClient::new(rpc_url.into()),
        }
    }

    pub async fn latest_slot(&self) -> Result<u64, ClientError> {
        retry(|| self.client.get_slot()).await
    }

    pub async fn get_block(&self, slot: u64) -> Result<UiConfirmedBlock, ClientError> {
        retry(|| {
            self.client.get_block_with_config(
                slot,
                RpcBlockConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    transaction_details: Some(TransactionDetails::Full),
                    rewards: Some(false),
                    commitment: None,
                    max_supported_transaction_version: Some(1),
                },
            )
        })
        .await
    }

    pub async fn get_finalized_blocks(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<u64>, ClientError> {
        retry(|| self.client.get_blocks(start_slot, Some(end_slot))).await
    }
}
