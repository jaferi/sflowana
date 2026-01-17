use async_trait::async_trait;
use std::sync::Mutex;

#[async_trait]
pub trait Checkpoint: Send + Sync {
    async fn load(&self) -> Result<Option<u64>, CheckpointError>;

    async fn save(&self, next_slot: u64) -> Result<(), CheckpointError>;
}

#[derive(Debug)]
pub enum CheckpointError {
    IoError(std::io::Error),
    Other(String),
}

pub struct MemoryCheckpoint {
    slot: Mutex<Option<u64>>,
}

impl MemoryCheckpoint {
    pub fn new() -> Self {
        Self {
            slot: Mutex::new(None),
        }
    }
}

impl Default for MemoryCheckpoint {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Checkpoint for MemoryCheckpoint {
    async fn load(&self) -> Result<Option<u64>, CheckpointError> {
        let slot = self
            .slot
            .lock()
            .map_err(|_| CheckpointError::Other("checkpoint mutex poisoned".into()))?;

        Ok(*slot)
    }

    async fn save(&self, next_slot: u64) -> Result<(), CheckpointError> {
        let mut checkpoint = self
            .slot
            .lock()
            .map_err(|_| CheckpointError::Other("checkpoint mutex poisoned".into()))?;

        *checkpoint = Some(next_slot);
        Ok(())
    }
}
