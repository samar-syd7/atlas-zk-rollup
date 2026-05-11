use halo2curves::bn256::Fr;

use prover::{
    circuit::PrivacySwapCircuit,
    witness::SwapWitness,
    zk::generate_proof,
};

fn main() {

    let owner =
        Fr::from(10);

    let amount =
        Fr::from(50);

    let secret =
        Fr::from(7);

    let commitment =
        owner + amount + secret;

    let nullifier =
        secret * secret;

    let witness =
        SwapWitness {
            owner,
            amount,
            secret,
            commitment,
            nullifier,
        };

    let circuit =
        PrivacySwapCircuit {
            witness,
        };

    let public_inputs =
        vec![
            vec![
                commitment,
                nullifier,
            ]
        ];

    let proof =
        generate_proof(
            circuit,
            public_inputs,
        )
        .unwrap();

    println!(
        "proof bytes: {}",
        proof.proof.len()
    );
}