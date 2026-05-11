use halo2_proofs::{
    plonk::{
        keygen_pk,
        keygen_vk,
    },
    poly::commitment::Params,
};

use halo2curves::bn256::G1Affine;

use crate::circuit::PrivacySwapCircuit;

pub fn generate_keys(
    circuit: &PrivacySwapCircuit,
) {

    let params: Params<G1Affine> =
        Params::new(8);

    let vk =
        keygen_vk(&params, circuit)
            .expect("vk generation failed");

    let _pk =
        keygen_pk(&params, vk, circuit)
            .expect("pk generation failed");
}