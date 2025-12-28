use solana_client::client_error::ClientError;

use crate::solana::block::Block;
use crate::solana::checkpoint::{Checkpoint, CheckpointError, MemoryCheckpoint};
use crate::solana::processor::{ProcessError, Processor};
use crate::solana::rpc::RpcSource;

pub enum StartPosition {
    Latest,
    BeforeLatest,
    Slot(u64),
    Resume,
}

pub struct Ingestor {
    rpc_source: RpcSource,
    processor: Processor,
    checkpoint_store: Box<dyn Checkpoint>,
}

impl Ingestor {
    pub fn new(rpc_source: RpcSource) -> Self {
        let checkpoint_store = Box::new(MemoryCheckpoint::new());
        let processor = Processor::new();

        Self {
            rpc_source,
            processor,
            checkpoint_store,
        }
    }

    pub async fn ingest(&self, start_position: StartPosition) -> Result<(), IngestorError> {
        let mut current_slot = match start_position {
            StartPosition::Latest => self.rpc_source.latest_slot().await?,

            StartPosition::BeforeLatest => self.rpc_source.latest_slot().await?.saturating_sub(1),

            StartPosition::Slot(slot) => slot,

            StartPosition::Resume => self
                .checkpoint_store
                .load()
                .await?
                .ok_or(IngestorError::NoCheckpoint)?
                .checked_add(1)
                .ok_or(IngestorError::SlotOverflow)?,
        };

        loop {
            let block = self.rpc_source.get_block(current_slot).await?;

            println!(
                "Fetched block for slot {} with {} transactions",
                current_slot,
                block
                    .transactions
                    .as_ref()
                    .map(|txs| txs.len())
                    .unwrap_or(0)
            );

            // Process the block
            let block: Block = Block::from_ui_confirmed_block(current_slot, &block);

            println!(
                "Ingesting slot {} with {} transactions",
                block.slot,
                block.transactions.len()
            );

            self.processor.process(&block).await?;

            // Save the checkpoint
            self.checkpoint_store.save(current_slot).await?;

            // Move to the next slot
            current_slot += 1;
        }
    }
}

#[derive(Debug)]
pub enum IngestorError {
    Rpc(ClientError),
    Process(ProcessError),
    Checkpoint(CheckpointError),
    NoCheckpoint,
    SlotOverflow,
}

impl From<ClientError> for IngestorError {
    fn from(error: ClientError) -> Self {
        Self::Rpc(error)
    }
}

impl From<ProcessError> for IngestorError {
    fn from(error: ProcessError) -> Self {
        Self::Process(error)
    }
}

impl From<CheckpointError> for IngestorError {
    fn from(error: CheckpointError) -> Self {
        Self::Checkpoint(error)
    }
}
