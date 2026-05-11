use std::collections::HashMap;

use primitives::{
    tx::RollupTx,
    merkle::merkle_root,
};

pub struct State {

    pub balances:
        HashMap<[u8; 32], u64>,
}

impl State {

    pub fn new() -> Self {

        Self {
            balances: HashMap::new(),
        }
    }

    pub fn apply_tx(
        &mut self,
        tx: &RollupTx,
    ) -> bool {

        let sender_balance =
            self
                .balances
                .entry(tx.from)
                .or_insert(1000);

        /*
            Reject invalid transaction.
        */

        if *sender_balance < tx.amount {

            return false;
        }

        *sender_balance -= tx.amount;

        let receiver_balance =
            self
                .balances
                .entry(tx.to)
                .or_insert(0);

        *receiver_balance += tx.amount;

        true
    }

    pub fn state_root(
        &self,
    ) -> [u8; 32] {

        let leaves =
            self
                .balances
                .iter()
                .map(|(addr, balance)| {

                    let mut bytes =
                        Vec::new();

                    bytes.extend_from_slice(addr);

                    bytes.extend_from_slice(
                        &balance.to_le_bytes()
                    );

                    blake3::hash(&bytes)
                        .into()
                })
                .collect();

        merkle_root(leaves)
    }
}