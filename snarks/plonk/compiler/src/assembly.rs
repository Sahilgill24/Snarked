use ark_ff::PrimeField;

// Simplified PLONK assembly

#[derive(Clone, Debug)]
pub struct Gate<F: PrimeField> {
    pub q_l: F,
    pub q_r: F,
    pub q_o: F,
    pub q_m: F,
    pub q_c: F,
}

impl<F: PrimeField> Gate<F> {
    pub fn new(q_l: F, q_r: F, q_o: F, q_m: F, q_c: F) -> Self {
        Self { q_l, q_r, q_o, q_m, q_c }
    }
}
