use ark_ff::PrimeField;

// Simplified utilities for Groth16

pub fn compute_witness<F: PrimeField>(inputs: &[F]) -> Vec<F> {
    // Simple witness computation
    inputs.to_vec()
}

pub fn hash_to_field<F: PrimeField>(data: &[u8]) -> F {
    // Simplified hash to field
    F::from(data.len() as u64)
}
