use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

// Simplified PLONK primitives

#[derive(Clone, Debug)]
pub struct PlonkProof<P: Pairing> {
    pub a_commit: P::G1,
    pub b_commit: P::G1,
    pub c_commit: P::G1,
    pub z_commit: P::G1,
}

#[derive(Clone, Debug)]
pub struct ProvingKey<P: Pairing> {
    pub q_selectors: Vec<P::G1>,
    pub sigma_commits: Vec<P::G1>,
}

#[derive(Clone, Debug)]
pub struct VerifyingKey<P: Pairing> {
    pub q_selectors: Vec<P::G1>,
    pub sigma_commits: Vec<P::G1>,
}

#[derive(Clone, Debug)]
pub struct Witness<F: PrimeField> {
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub c: Vec<F>,
}

impl<F: PrimeField> Witness<F> {
    pub fn new(a: Vec<F>, b: Vec<F>, c: Vec<F>) -> Self {
        Self { a, b, c }
    }
}
