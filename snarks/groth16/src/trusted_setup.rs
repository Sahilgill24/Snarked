use crate::primitives::{ProvingKey, VerifyingKey};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::{One, PrimeField};

// Simplified trusted setup for Groth16

pub struct TrustedSetup<P: Pairing> {
    _phantom: std::marker::PhantomData<P>,
}

impl<P: Pairing> TrustedSetup<P> {
    /// Run trusted setup ceremony
    pub fn setup(tau: P::ScalarField, size: usize) -> (ProvingKey<P>, VerifyingKey<P>) {
        // Generate powers of tau
        let mut tau_powers = Vec::with_capacity(size);
        let mut current = P::ScalarField::one();

        for _ in 0..size {
            tau_powers.push(P::G1::generator().mul_bigint(current.into_bigint()));
            current = current * tau;
        }

        let alpha = P::G1::generator();
        let beta = P::G2::generator();
        let gamma = P::G2::generator();
        let delta = P::G2::generator();

        let pk = ProvingKey {
            alpha: alpha.clone(),
            beta: beta.clone(),
            delta: delta.clone(),
            tau_powers,
        };

        let vk = VerifyingKey {
            alpha,
            beta,
            gamma,
            delta,
        };

        (pk, vk)
    }
}
