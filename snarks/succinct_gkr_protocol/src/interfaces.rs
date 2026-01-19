use ark_ff::PrimeField;

use crate::primitives::{Circuit, GKRProof};

pub trait GKRProverInterface<F: PrimeField> {
    fn prove(circuit: &Circuit, input: &[F]) -> GKRProof<F>;
}

pub trait GKRVerifierInterface<F: PrimeField> {
    fn verify(circuit: &Circuit, proof: &GKRProof<F>, input: &[F]) -> bool;
}
