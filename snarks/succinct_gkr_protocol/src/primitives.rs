use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

// Simplified Succinct GKR primitives

#[derive(Clone, Debug)]
pub struct GKRProof<F: PrimeField, P: Pairing> {
    pub sumcheck_proofs: Vec<F>,
    pub evaluations: Vec<F>,
    pub openings: Vec<P::G1>,
}

#[derive(Clone, Debug)]
pub struct Circuit<F: PrimeField> {
    pub layers: Vec<Layer<F>>,
}

#[derive(Clone, Debug)]
pub struct Layer<F: PrimeField> {
    pub gates: Vec<Gate<F>>,
}

#[derive(Clone, Debug)]
pub struct Gate<F: PrimeField> {
    pub gate_type: GateType,
    pub inputs: Vec<usize>,
    _phantom: std::marker::PhantomData<F>,
}

#[derive(Clone, Debug)]
pub enum GateType {
    Add,
    Mul,
}

impl<F: PrimeField> Circuit<F> {
    pub fn new(layers: Vec<Layer<F>>) -> Self {
        Self { layers }
    }
}

impl<F: PrimeField> Layer<F> {
    pub fn new(gates: Vec<Gate<F>>) -> Self {
        Self { gates }
    }
}
