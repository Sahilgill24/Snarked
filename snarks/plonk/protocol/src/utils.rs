use ark_ff::PrimeField;

// Simplified PLONK utilities

pub fn compute_permutation<F: PrimeField>(values: &[F]) -> Vec<F> {
    values.to_vec()
}

pub fn evaluate_vanishing_poly<F: PrimeField>(x: F, n: usize) -> F {
    x.pow(&[n as u64]) - F::ONE
}
