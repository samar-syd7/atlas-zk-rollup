use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice,
        Circuit,
        Column,
        ConstraintSystem,
        Error,
        Instance,
        Selector,
    },
    poly::Rotation,
};

use halo2curves::bn256::Fr;

use crate::witness::SwapWitness;

#[derive(Clone, Debug)]
pub struct SwapConfig {
    pub advice: [Column<Advice>; 5],

    pub instance: Column<Instance>,

    pub selector: Selector,
}

#[derive(Clone, Debug)]
pub struct PrivacySwapCircuit {
    pub witness: SwapWitness,
}

impl SwapConfig {
    pub fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let advice = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];

        let instance = meta.instance_column();

        for column in advice.iter() {
            meta.enable_equality(*column);
        }

        meta.enable_equality(instance);

        meta.enable_equality(advice[3]);
        meta.enable_equality(advice[4]);

        let selector = meta.selector();

        /*
            Layout:

            advice[0] => owner
            advice[1] => amount
            advice[2] => secret
            advice[3] => commitment
            advice[4] => nullifier
        */

        meta.create_gate("commitment constraint", |meta| {
            let s = meta.query_selector(selector);

            let owner = meta.query_advice(advice[0], Rotation::cur());
            let amount = meta.query_advice(advice[1], Rotation::cur());
            let secret = meta.query_advice(advice[2], Rotation::cur());

            let commitment =
                meta.query_advice(advice[3], Rotation::cur());

            /*
                Simplified hash relation.

                Real systems use Poseidon.
                We use arithmetic composition here
                for clarity + compilability.
            */

            vec![
                s * ((owner + amount + secret) - commitment)
            ]
        });

        meta.create_gate("nullifier constraint", |meta| {
            let s = meta.query_selector(selector);

            let secret = meta.query_advice(advice[2], Rotation::cur());

            let nullifier =
                meta.query_advice(advice[4], Rotation::cur());

            vec![
                s * ((secret.clone() * secret) - nullifier)
            ]
        });

        Self {
            advice,
            instance,
            selector,
        }
    }
}

impl Circuit<Fr> for PrivacySwapCircuit {
    type Config = SwapConfig;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        self.clone()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        SwapConfig::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {

        let (commitment_cell, nullifier_cell) =
            layouter.assign_region(
                || "privacy swap region",
                |mut region| {

                    config.selector.enable(&mut region, 0)?;

                    region.assign_advice(
                        || "owner",
                        config.advice[0],
                        0,
                        || Value::known(self.witness.owner),
                    )?;

                    region.assign_advice(
                        || "amount",
                        config.advice[1],
                        0,
                        || Value::known(self.witness.amount),
                    )?;

                    region.assign_advice(
                        || "secret",
                        config.advice[2],
                        0,
                        || Value::known(self.witness.secret),
                    )?;

                    let commitment_cell =
                        region.assign_advice(
                            || "commitment",
                            config.advice[3],
                            0,
                            || Value::known(self.witness.commitment),
                        )?;

                    let nullifier_cell =
                        region.assign_advice(
                            || "nullifier",
                            config.advice[4],
                            0,
                            || Value::known(self.witness.nullifier),
                        )?;

                    Ok((commitment_cell, nullifier_cell))
                },
            )?;

        layouter.constrain_instance(
            commitment_cell.cell(),
            config.instance,
            0,
        )?;

        layouter.constrain_instance(
            nullifier_cell.cell(),
            config.instance,
            1,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use halo2_proofs::dev::MockProver;

    #[test]
    fn test_privacy_swap_circuit() {

        let owner = Fr::from(10);

        let amount = Fr::from(50);

        let secret = Fr::from(7);

        let commitment =
            owner + amount + secret;

        let nullifier =
            secret * secret;

        let witness = SwapWitness {
            owner,
            amount,
            secret,
            commitment,
            nullifier,
        };

        let circuit =
            PrivacySwapCircuit { witness };

        let prover =
            MockProver::run(
                8,
                &circuit,
                vec![vec![commitment, nullifier]],
            )
            .unwrap();

        prover.assert_satisfied();
    }
}