use solana_client::rpc_response::UiConfirmedBlock;
use solana_client::rpc_response::EncodedTransaction;
use crate::solana::transaction::Transaction;

pub struct Block {
    pub slot: u64,
    pub blockhash: String,
    pub parent_slot: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(slot: u64, blockhash: String, parent_slot: u64, transactions: Vec<Transaction>) -> Self {
        Self {
            slot,
            blockhash,
            parent_slot,
            transactions,
        }
    }

    pub fn from_ui_confirmed_block(slot: u64, block: &UiConfirmedBlock) -> Self {
        let transactions = block.transactions.as_ref().map(|transactions| {
            transactions.iter().enumerate().filter_map(|(index, txn)| {
                match &txn.transaction {
                    EncodedTransaction::Json(tx) => {
                        tx.signatures.first().map(|signature| Transaction {
                            signature: signature.clone(),
                            index,
                        })
                    }
                    _ => None,
                }
            }).collect()
        }).unwrap_or_default();

        Self {
            slot,
            blockhash: block.blockhash.clone(),
            parent_slot: block.parent_slot,
            transactions,
        }
    }
    
}
