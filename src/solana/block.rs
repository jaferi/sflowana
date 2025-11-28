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
                                                let program_id = message.account_keys
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
