use anyhow::Result;

use ethers::{
    prelude::*,
    providers::{
        Http,
        Provider,
    },
    signers::LocalWallet,
};

use std::{
    sync::Arc,
    time::Duration,
};

abigen!(
    RollupVerifier,
    "./crates/contracts/out/RollupVerifier.sol/RollupVerifier.json"
);

pub struct EthereumClient {

    contract:
        RollupVerifier<
            SignerMiddleware<
                Provider<Http>,
                LocalWallet
            >
        >,
}

impl EthereumClient {

    pub async fn new(
        rpc_url: &str,
        private_key: &str,
        contract_address: &str,
    ) -> Result<Self> {

        /*
            Ethereum RPC connection.
        */

        let provider =
            Provider::<Http>::try_from(rpc_url)?
                .interval(
                    Duration::from_millis(10)
                );

        /*
            Wallet for signing txs.
        */

        let wallet: LocalWallet =
            private_key.parse()?;

        /*
            Attach chain ID.
        */

        let wallet =
            wallet.with_chain_id(11155111u64);

        /*
            Combine provider + signer.
        */

        let client =
            Arc::new(
                SignerMiddleware::new(
                    provider,
                    wallet,
                )
            );

        /*
            Smart contract instance.
        */

        let address: Address =
            contract_address.parse()?;

        let contract =
            RollupVerifier::new(
                address,
                client,
            );

        Ok(
            Self {
                contract,
            }
        )
    }

    pub async fn submit_batch(

        &self,

        proof: Vec<u8>,

        batch_hash: [u8; 32],

        new_root: [u8; 32],

    ) -> Result<()> {

        let call =
            self
                .contract
                .verify_batch(
                    proof.into(),
                    batch_hash,
                    new_root,
                );

        let pending_tx =
            call
                .send()
                .await?;

        let receipt =
            pending_tx.await?;

        println!(
            "batch submitted: {:?}",
            receipt
        );

        Ok(())
    }
}