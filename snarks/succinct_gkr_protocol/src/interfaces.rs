use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use crate::primitives::{Circuit, GKRProof};

// Simplified GKR interfaces

pub trait GKRProverInterface<F: PrimeField, P: Pairing> {
    fn prove(circuit: &Circuit<F>, witness: &[F]) -> GKRProof<F, P>;
}

pub trait GKRVerifierInterface<F: PrimeField, P: Pairing> {
    fn verify(circuit: &Circuit<F>, proof: &GKRProof<F, P>, public_input: &[F]) -> bool;
}
