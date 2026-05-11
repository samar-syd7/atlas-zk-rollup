use serde::{
    Serialize,
    Deserialize,
};

use crate::tx::RollupTx;
use blake3::Hasher;

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct RollupBatch {

    pub batch_id: u64,

    pub txs: Vec<RollupTx>,

    pub prev_state_root: [u8; 32],

    pub new_state_root: [u8; 32],
}

impl RollupBatch {

    pub fn hash(
        &self,
    ) -> [u8; 32] {

        let mut hasher =
            Hasher::new();

        hasher.update(
            &self.batch_id.to_le_bytes()
        );

        hasher.update(
            &self.prev_state_root
        );

        hasher.update(
            &self.new_state_root
        );

        for tx in &self.txs {

            hasher.update(
                &tx.hash()
            );
        }

        let hash =
            hasher.finalize();

        *hash.as_bytes()
    }
}