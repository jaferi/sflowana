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

    async fn resolve_start_position(
        &self,
        start_position: StartPosition,
    ) -> Result<u64, IngestorError> {
        match start_position {
            StartPosition::Latest => Ok(self.rpc_source.latest_slot().await?),

            StartPosition::BeforeLatest => {
                Ok(self.rpc_source.latest_slot().await?.saturating_sub(1))
            }

            StartPosition::Slot(slot) => Ok(slot),

            StartPosition::Resume => self
                .checkpoint_store
                .load()
                .await?
                .ok_or(IngestorError::NoCheckpoint)
                .and_then(|slot| slot.checked_add(1).ok_or(IngestorError::SlotOverflow)),
        }
    }

    pub async fn ingest(&self, start_position: StartPosition) -> Result<(), IngestorError> {
        let mut current_slot = self.resolve_start_position(start_position).await?;

        loop {
            let latest_slot = self.rpc_source.latest_slot().await?;

            if current_slot > latest_slot {
                // let's wait
                continue;
            }

            let end_slot = current_slot.saturating_add(1_000).min(latest_slot);

            let finalized_blocks = self
                .rpc_source
                .get_finalized_blocks(current_slot, end_slot)
                .await?;

            if finalized_blocks.is_empty() {
                // No finalized blocks in the range, let's wait
                continue;
            }

            for slot_id in finalized_blocks {
                let block = self.rpc_source.get_block(slot_id).await?;

                println!(
                    "Fetched block for slot {} with {} transactions",
                    slot_id,
                    block
                        .transactions
                        .as_ref()
                        .map(|txs| txs.len())
                        .unwrap_or(0)
                );

                // Process the block
                let block: Block = Block::from_ui_confirmed_block(slot_id, &block);

                println!(
                    "Ingesting slot {} with {} transactions",
                    block.slot,
                    block.transactions.len()
                );

                self.processor.process(&block).await?;

                // Save the checkpoint
                self.checkpoint_store.save(slot_id + 1).await?;
            }

            current_slot = end_slot
                .checked_add(1)
                .ok_or(IngestorError::SlotOverflow)?;
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
