use primitives::{
    batch::RollupBatch,
    tx::RollupTx,
};

use crate::state::State;

pub struct Batcher {

    batch_id: u64,
}

impl Batcher {

    pub fn new() -> Self {

        Self {
            batch_id: 0,
        }
    }

    pub fn create_batch(
        &mut self,
        txs: Vec<RollupTx>,
        state: &mut State,
    ) -> RollupBatch {

        let prev_root =
            state.state_root();

        let mut valid_txs = Vec::new();

        for tx in txs {

            let success =
                state.apply_tx(&tx);

            if success {
                valid_txs.push(tx);
            }
        }

        let new_root =
            state.state_root();

        self.batch_id += 1;

        RollupBatch {
            batch_id: self.batch_id,
            txs: valid_txs,
            prev_state_root: prev_root,
            new_state_root: new_root,
        }
    }
}