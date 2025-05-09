use substreams::{log, Hex};
use substreams::store::{StoreAdd, Deltas};
use substreams_solana::pb::sf::solana::r#type::v1::{Transaction, Block};

use pagcrypto::Transactions;

#[substreams::handlers::map]
fn map_pagcrypto_tx(block: Block) -> Result<Transactions, substreams::errors::Error> {
    let mut txs = vec![];

    for entry in block.transactions.iter() {
        if let Some(meta) = &entry.meta {
            if meta.err.is_none() {
                for inst in &entry.transaction.message.instructions {
                    if let Some(memo) = &inst.parsed {
                        if memo.contains("paycrypto:") {
                            txs.push(pagcrypto::Transaction {
                                signature: entry.transaction.signatures.get(0).unwrap_or(&String::new()).clone(),
                                slot: block.slot,
                                memo: memo.clone(),
                                fee: meta.fee as u64,
                                success: true,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(Transactions { items: txs })
}