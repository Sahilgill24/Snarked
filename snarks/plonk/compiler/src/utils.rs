use ark_ff::PrimeField;

// Simplified PLONK compiler utilities

pub fn compile_circuit<F: PrimeField>(gates: &[F]) -> Vec<F> {
    gates.to_vec()
}
