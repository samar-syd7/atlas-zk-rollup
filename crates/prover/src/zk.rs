use anyhow::Result;

use halo2_proofs::{

    plonk::{
        create_proof,
        keygen_pk,
        keygen_vk,
    },

    poly::{
        commitment::Params,
    },

    transcript::{
        Blake2bWrite,
        Challenge255,
    },
};

use halo2curves::{
    bn256::{
        Fr,
        G1Affine,
    },
};

use rand::rngs::OsRng;

use crate::circuit::PrivacySwapCircuit;

pub struct GeneratedProof {

    pub proof: Vec<u8>,
}

pub fn generate_proof(

    circuit: PrivacySwapCircuit,

    public_inputs: Vec<Vec<Fr>>,

) -> Result<GeneratedProof> {

    /*
        Security parameter.

        Higher k =
        larger circuits.
    */
    let k = 8;

    /*
        Polynomial commitment setup.
    */
    let params:
        Params<G1Affine> =
            Params::new(k);

    /*
        Verification key.
    */
    let vk =
        keygen_vk(
            &params,
            &circuit,
        )?;

    /*
        Proving key.
    */
    let pk =
        keygen_pk(
            &params,
            vk,
            &circuit,
        )?;

    /*
        Transcript accumulates
        cryptographic proof data.
    */
    let mut transcript =
        Blake2bWrite::<
            _,
            G1Affine,
            Challenge255<G1Affine>,
        >::init(vec![]);

    /*
        Actual proof generation.
    */
    create_proof::<
        G1Affine,
        Challenge255<G1Affine>,
        _,
        Blake2bWrite<
            Vec<u8>,
            G1Affine,
            Challenge255<G1Affine>,
        >,
        PrivacySwapCircuit,
    >(
        &params,
        &pk,
        &[circuit],
        &[&public_inputs
            .iter()
            .map(|x| x.as_slice())
            .collect::<Vec<_>>()],
        OsRng,
        &mut transcript,
    )?;

    let proof =
        transcript.finalize();

    Ok(
        GeneratedProof {
            proof,
        }
    )
}