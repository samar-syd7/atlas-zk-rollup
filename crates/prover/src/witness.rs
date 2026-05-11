use halo2curves::bn256::Fr;

#[derive(Clone, Debug)]
pub struct SwapWitness {
    pub owner: Fr,
    pub amount: Fr,
    pub secret: Fr,

    pub commitment: Fr,
    pub nullifier: Fr,
}