use crate::primitives::{Proof, ProvingKey, VerifyingKey};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

// Simplified interfaces for Groth16

pub trait ProverInterface<P: Pairing> {
    fn prove(pk: &ProvingKey<P>, witness: &[P::ScalarField]) -> Proof<P>;
}

pub trait VerifierInterface<P: Pairing> {
    fn verify(vk: &VerifyingKey<P>, proof: &Proof<P>, public_input: &[P::ScalarField]) -> bool;
}
