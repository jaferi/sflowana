use bs58;

use solana_client::rpc_response::UiConfirmedBlock;

use solana_transaction_status_client_types::{
    option_serializer::OptionSerializer, EncodedTransaction, EncodedTransactionWithStatusMeta,
    UiMessage,
};

use crate::solana::transaction::{Instruction, Transaction};

pub struct Block {
    pub slot: u64,
    pub blockhash: String,
    pub parent_slot: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        slot: u64,
        blockhash: String,
        parent_slot: u64,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            slot,
            blockhash,
            parent_slot,
            transactions,
        }
    }

    pub fn from_ui_confirmed_block(slot: u64, block: &UiConfirmedBlock) -> Self {
        let transactions = block
            .transactions
            .as_ref()
            .map(|transactions: &Vec<EncodedTransactionWithStatusMeta>| {
                transactions
                    .iter()
                    .enumerate()
                    .filter_map(|(index, txn)| {
                        match &txn.transaction {
                            EncodedTransaction::Json(tx) => {
                                let instructions = match &tx.message {
                                    UiMessage::Raw(message) => {
                                        let mut resolved_accounts = message.account_keys.clone();

                                        // also add dynamic accounts from the transaction meta if available
                                        if let Some(meta) = &txn.meta {
                                            if let OptionSerializer::Some(loaded) =
                                                &meta.loaded_addresses
                                            {
                                                resolved_accounts
                                                    .extend(loaded.writable.iter().cloned());
                                                resolved_accounts
                                                    .extend(loaded.readonly.iter().cloned());
                                            }
                                        }

                                        message
                                            .instructions
                                            .iter()
                                            .map(|ix| {
                                                let program_id = resolved_accounts
                                                    [ix.program_id_index as usize]
                                                    .clone();

                                                let accounts = ix
                                                    .accounts
                                                    .iter()
                                                    .map(|&i| resolved_accounts[i as usize].clone())
                                                    .collect();

                                                let data = bs58::decode(&ix.data)
                                                    .into_vec()
                                                    .unwrap_or_default();

                                                Instruction {
                                                    program_id,
                                                    accounts,
                                                    data,
                                                }
                                            })
                                            .collect()
                                    }
                                    _ => Vec::new(),
                                };

                                tx.signatures.first().map(|signature| Transaction {
                                    signature: signature.clone(),
                                    index,
                                    instructions,
                                })
                            }
                            _ => None,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self {
            slot,
            blockhash: block.blockhash.clone(),
            parent_slot: block.parent_slot,
            transactions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_creates_block_with_expected_fields() {
        let block = Block::new(
            100,
            "blockhash".to_owned(),
            99,
            vec![],
        );

        assert_eq!(block.slot, 100);
        assert_eq!(block.blockhash, "blockhash");
        assert_eq!(block.parent_slot, 99);
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn converts_block_without_transactions() {
        let ui_block = UiConfirmedBlock {
            previous_blockhash: "previous".to_owned(),
            blockhash: "current".to_owned(),
            parent_slot: 99,
            transactions: None,
            signatures: None,
            rewards: None,
            num_reward_partitions: None,
            block_time: None,
            block_height: None,
        };

        let block = Block::from_ui_confirmed_block(100, &ui_block);

        assert_eq!(block.slot, 100);
        assert_eq!(block.blockhash, "current");
        assert_eq!(block.parent_slot, 99);
        assert!(block.transactions.is_empty());
    }

    fn ui_block(transactions: serde_json::Value) -> UiConfirmedBlock {
        serde_json::from_value(json!({
            "previousBlockhash": "previous",
            "blockhash": "current",
            "parentSlot": 99,
            "transactions": transactions,
            "signatures": null,
            "rewards": null,
            "numRewardPartitions": null,
            "blockTime": null,
            "blockHeight": null
        }))
        .expect("valid UiConfirmedBlock fixture")
    }

    #[test]
    fn preserves_block_metadata() {
        let source = ui_block(json!(null));

        let block = Block::from_ui_confirmed_block(100, &source);

        assert_eq!(block.slot, 100);
        assert_eq!(block.blockhash, "current");
        assert_eq!(block.parent_slot, 99);
    }

    #[test]
    fn returns_empty_transactions_when_transactions_are_missing() {
        let source = ui_block(json!(null));

        let block = Block::from_ui_confirmed_block(100, &source);

        assert!(block.transactions.is_empty());
    }

    #[test]
    fn returns_empty_transactions_for_empty_transaction_list() {
        let source = ui_block(json!([]));

        let block = Block::from_ui_confirmed_block(100, &source);

        assert!(block.transactions.is_empty());
    }
}