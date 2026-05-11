use prover::prover::{
    ProofJob,
    ProverEngine,
};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let engine =
        ProverEngine::new();

    engine.submit(
        ProofJob {
            batch_id: 42,
            tx_count: 500,
        }
    )
    .await
    .unwrap();

    tokio::time::sleep(
        std::time::Duration::from_secs(5)
    )
    .await;
}