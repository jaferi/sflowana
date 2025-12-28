use crate::solana::block::Block;

pub struct Processor;

impl Processor {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(&self, block: &Block) -> Result<(), ProcessError> {
        println!(
            "Processing slot {} with {} transactions",
            block.slot,
            block.transactions.len()
        );

        Ok(())
    }
}

#[derive(Debug)]
pub enum ProcessError {
    Other(String),
}
