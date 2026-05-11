mod ethereum;
use ethereum::EthereumClient;
use dotenvy::dotenv;
use std::env;
mod mempool;
mod state;
mod batcher;
mod db;
mod rpc;

use tokio::time::{
    sleep,
    Duration,
};

use tracing::info;

use primitives::tx::RollupTx;

use mempool::Mempool;
use state::State;
use batcher::Batcher;
use db::Database;
use rpc::start_rpc;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    dotenv().ok();
    tracing_subscriber::fmt::init();


    let mempool =
        Arc::new(
            Mempool::new()
        );

    let mut state =
        State::new();

    let mut batcher =
        Batcher::new();

    let rpc_url =
    env::var("SEPOLIA_RPC_URL")
        .unwrap();

    let private_key =
        env::var("PRIVATE_KEY")
            .unwrap();

    let contract_address =
        env::var("CONTRACT_ADDRESS")
            .unwrap();

    let eth_client =
        EthereumClient::new(
            &rpc_url,
            &private_key,
            &contract_address,
        )
        .await
        .unwrap();

    let db =
        Database::new(
            "./rollup_db"
        )
        .unwrap();

    let rpc_mempool =
        mempool.clone();

    tokio::spawn(async move {

        start_rpc(
            rpc_mempool
        )
        .await;
    });

    /*
        Fake user transactions.
    */

    for i in 0..100 {

        let tx = RollupTx {

            from: [1u8; 32],

            to: [i as u8; 32],

            amount: 10,

            nonce: i,
        };

        mempool.insert(tx);
    }

    /*
        For loop is for testing to run only 4 batches for testnet eth submission
    */

    for _ in 0..4 { // 

        let txs =
            mempool.drain(25);

        if txs.is_empty() {

            info!("no pending txs");

            sleep(
                Duration::from_secs(2)
            )
            .await;

            continue;
        }

        let batch =
            batcher.create_batch(
                txs,
                &mut state,
            );

        info!(
            "created batch {} with {} txs",
            batch.batch_id,
            batch.txs.len(),
        );

        info!(
            "new state root: {:?}",
            batch.new_state_root,
        );

        let proof =
            vec![1, 2, 3];

        /*
            Fake batch hash for now.
        */
        let batch_hash =
            batch.hash();

        eth_client
            .submit_batch(
                proof,
                batch_hash,
                batch.new_state_root,
            )
            .await
            .unwrap();

        db.save_batch(&batch)
            .unwrap();

        info!(
            "batch submitted to Ethereum"
        );

        sleep(
            Duration::from_secs(3)
        )
        .await;
    }

    /*
        Below is the actual production loop for real time sequencing and batch submission to eth, but for testing we are running only 4 batches above
    */

    // loop {

    //     let txs =
    //         mempool.drain(25);

    //     if txs.is_empty() {

    //         info!("no pending txs");

    //         sleep(
    //             Duration::from_secs(2)
    //         )
    //         .await;

    //         continue;
    //     }

    //     let batch =
    //         batcher.create_batch(
    //             txs,
    //             &mut state,
    //         );

    //     info!(
    //         "created batch {} with {} txs",
    //         batch.batch_id,
    //         batch.txs.len(),
    //     );

    //     info!(
    //         "new state root: {:?}",
    //         batch.new_state_root,
    //     );

    //     let proof =
    //         vec![1, 2, 3];

    //     /*
    //         Fake batch hash for now.
    //     */
    //     let batch_hash =
    //         blake3::hash(
    //             &batch.batch_id.to_le_bytes()
    //         );

    //     eth_client
    //         .submit_batch(
    //             proof,
    //             *batch_hash.as_bytes(),
    //             batch.new_state_root,
    //         )
    //         .await
    //         .unwrap();

    //     info!(
    //         "batch submitted to Ethereum"
    //     );

    //     sleep(
    //         Duration::from_secs(3)
    //     )
    //     .await;
    // }
}