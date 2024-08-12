use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

// Simplified primitives for Groth16 SNARK

#[derive(Clone, Debug)]
pub struct Proof<P: Pairing> {
    pub a: P::G1,
    pub b: P::G2,
    pub c: P::G1,
}

#[derive(Clone, Debug)]
pub struct ProvingKey<P: Pairing> {
    pub alpha: P::G1,
    pub beta: P::G2,
    pub delta: P::G2,
    pub tau_powers: Vec<P::G1>,
}

#[derive(Clone, Debug)]
pub struct VerifyingKey<P: Pairing> {
    pub alpha: P::G1,
    pub beta: P::G2,
    pub gamma: P::G2,
    pub delta: P::G2,
}

impl<P: Pairing> Proof<P> {
    pub fn new(a: P::G1, b: P::G2, c: P::G1) -> Self {
        Self { a, b, c }
    }
}
