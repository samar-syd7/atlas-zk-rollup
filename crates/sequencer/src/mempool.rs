use dashmap::DashMap;

use primitives::tx::RollupTx;

pub struct Mempool {

    txs: DashMap<[u8; 32], RollupTx>,
}

impl Mempool {

    pub fn new() -> Self {

        Self {
            txs: DashMap::new(),
        }
    }

    pub fn insert(
        &self,
        tx: RollupTx,
    ) {

        self.txs.insert(
            tx.hash(),
            tx,
        );
    }

    pub fn drain(
        &self,
        max: usize,
    ) -> Vec<RollupTx> {

        let keys: Vec<[u8; 32]> =
            self
                .txs
                .iter()
                .take(max)
                .map(|entry| *entry.key())
                .collect();

        let mut drained =
            Vec::new();

        for key in keys {

            if let Some((_, tx)) =
                self.txs.remove(&key)
            {
                drained.push(tx);
            }
        }

        drained
    }
}