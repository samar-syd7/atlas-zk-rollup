use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct RollupTx {

    pub from: [u8; 32],

    pub to: [u8; 32],

    pub amount: u64,

    pub nonce: u64,
}

impl RollupTx {

    pub fn hash(&self) -> [u8; 32] {

        let encoded =
            serde_json::to_vec(self)
                .unwrap();

        let hash =
            blake3::hash(&encoded);

        *hash.as_bytes()
    }
}