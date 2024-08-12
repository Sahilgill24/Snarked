use ark_ff::PrimeField;

// Simplified PLONK program

#[derive(Clone, Debug)]
pub struct Program<F: PrimeField> {
    pub gates: Vec<F>,
    pub size: usize,
}

impl<F: PrimeField> Program<F> {
    pub fn new(size: usize) -> Self {
        Self {
            gates: Vec::new(),
            size,
        }
    }
}
