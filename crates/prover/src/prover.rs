use std::{
    time::Instant,
};

use anyhow::Result;

use rayon::prelude::*;

use tokio::{
    sync::mpsc,
    task,
};

use tracing::{
    info,
    instrument,
};

use halo2curves::bn256::Fr;

use crate::{
    circuit::PrivacySwapCircuit,
    witness::SwapWitness,
};

#[derive(Clone, Debug)]
pub struct ProofJob {
    pub batch_id: u64,
    pub tx_count: usize,
}

#[derive(Clone, Debug)]
pub struct ProofResult {
    pub batch_id: u64,
    pub proving_time_ms: u128,
}

pub struct ProverEngine {
    sender: mpsc::Sender<ProofJob>,
}

impl ProverEngine {

    pub fn new() -> Self {

        let (tx, mut rx) =
            mpsc::channel::<ProofJob>(1024);

        task::spawn(async move {

            while let Some(job) = rx.recv().await {

                tokio::spawn(async move {

                    if let Err(err) =
                        process_job(job).await
                    {
                        tracing::error!(
                            "proof job failed: {:?}",
                            err
                        );
                    }
                });
            }
        });

        Self {
            sender: tx,
        }
    }

    pub async fn submit(
        &self,
        job: ProofJob,
    ) -> Result<()> {

        self.sender.send(job).await?;

        Ok(())
    }
}

#[instrument]
async fn process_job(
    job: ProofJob,
) -> Result<ProofResult> {

    info!(
        "starting proof generation for batch {}",
        job.batch_id
    );

    let start = Instant::now();

    /*
        Parallel witness generation.

        This demonstrates:
        - rayon usage
        - CPU parallelism
        - zk preprocessing
    */

    let witnesses: Vec<SwapWitness> =
        (0..job.tx_count)
            .into_par_iter()
            .map(generate_witness)
            .collect();

    /*
        Build circuits in parallel.
    */

    let circuits: Vec<PrivacySwapCircuit> =
        witnesses
            .into_par_iter()
            .map(|witness| {
                PrivacySwapCircuit {
                    witness,
                }
            })
            .collect();

    /*
        Simulated proving workload.

        Later replaced with:
        - create_proof()
        - transcript logic
        - KZG commitments
    */

    circuits
        .par_iter()
        .for_each(|_| {
            heavy_compute_simulation();
        });

    let elapsed =
        start.elapsed().as_millis();

    info!(
        "proof completed in {} ms",
        elapsed
    );

    Ok(
        ProofResult {
            batch_id: job.batch_id,
            proving_time_ms: elapsed,
        }
    )
}

fn generate_witness(
    i: usize,
) -> SwapWitness {

    let owner =
        Fr::from(i as u64 + 1);

    let amount =
        Fr::from(100);

    let secret =
        Fr::from((i * 7) as u64 + 11);

    let commitment =
        owner + amount + secret;

    let nullifier =
        secret * secret;

    SwapWitness {
        owner,
        amount,
        secret,
        commitment,
        nullifier,
    }
}

fn heavy_compute_simulation() {

    /*
        Simulate expensive proving arithmetic.

        This mimics:
        - FFTs
        - MSMs
        - polynomial operations
    */

    let mut acc = 0u64;

    for i in 0..5_000_000 {
        acc = acc.wrapping_add(i);
    }

    std::hint::black_box(acc);
}

pub fn process_job_sync(
    job: ProofJob,
) -> Result<ProofResult> {

    let runtime =
        tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        process_job(job).await
    })
}