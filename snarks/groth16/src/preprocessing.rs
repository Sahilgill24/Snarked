use ark_ff::PrimeField;

// Simplified preprocessing for Groth16

#[derive(Clone, Debug)]
pub struct Circuit<F: PrimeField> {
    pub constraints: Vec<Constraint<F>>,
}

#[derive(Clone, Debug)]
pub struct Constraint<F: PrimeField> {
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub c: Vec<F>,
}

impl<F: PrimeField> Circuit<F> {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, a: Vec<F>, b: Vec<F>, c: Vec<F>) {
        self.constraints.push(Constraint { a, b, c });
    }
}
