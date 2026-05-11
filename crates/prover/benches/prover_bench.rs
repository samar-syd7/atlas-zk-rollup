use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

use prover::prover::{
    ProofJob,
    process_job_sync,
};

fn benchmark_prover(
    c: &mut Criterion,
) {

    c.bench_function(
        "prove_100_txs",
        |b| {
            b.iter(|| {

                process_job_sync(
                    ProofJob {
                        batch_id: 1,
                        tx_count: 100,
                    }
                )
                .unwrap();
            });
        }
    );
}

criterion_group!(
    benches,
    benchmark_prover
);

criterion_main!(benches);