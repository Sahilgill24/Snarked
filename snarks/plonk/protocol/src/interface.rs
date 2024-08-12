use ark_ec::pairing::Pairing;
use plonk_core::primitives::{PlonkProof, ProvingKey, VerifyingKey, Witness};

// Simplified PLONK interfaces

pub trait PlonkProverInterface<P: Pairing> {
    fn prove(pk: &ProvingKey<P>, witness: &Witness<P::ScalarField>) -> PlonkProof<P>;
}

pub trait PlonkVerifierInterface<P: Pairing> {
    fn verify(vk: &VerifyingKey<P>, proof: &PlonkProof<P>, public_input: &[P::ScalarField]) -> bool;
}
