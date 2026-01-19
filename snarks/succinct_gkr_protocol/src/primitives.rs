use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateType {
    Add,
    Mul,
}

/// A gate reads two wires from the layer below and combines them.
#[derive(Clone, Copy, Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub left: usize,
    pub right: usize,
}

impl Gate {
    pub fn add(left: usize, right: usize) -> Self {
        Gate { gate_type: GateType::Add, left, right }
    }

    pub fn mul(left: usize, right: usize) -> Self {
        Gate { gate_type: GateType::Mul, left, right }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub gates: Vec<Gate>,
}

impl Layer {
    pub fn new(gates: Vec<Gate>) -> Self {
        Layer { gates }
    }
}

/// A layered arithmetic circuit. `layers[0]` is the output layer; each layer's
/// gates index into the layer below it, and the bottom layer indexes into the
/// input. Every layer (and the input) must have a power-of-two width.
#[derive(Clone, Debug)]
pub struct Circuit {
    pub layers: Vec<Layer>,
}

impl Circuit {
    pub fn new(layers: Vec<Layer>) -> Self {
        Circuit { layers }
    }

    /// Evaluate the circuit, returning every layer's wire values.
    /// `values[0]` is the output and `values[depth]` is the input.
    pub fn evaluate<F: PrimeField>(&self, input: &[F]) -> Vec<Vec<F>> {
        let depth = self.layers.len();
        let mut values = vec![Vec::new(); depth + 1];
        values[depth] = input.to_vec();

        for i in (0..depth).rev() {
            let below = &values[i + 1];
            let out = self.layers[i]
                .gates
                .iter()
                .map(|gate| {
                    let l = below[gate.left];
                    let r = below[gate.right];
                    match gate.gate_type {
                        GateType::Add => l + r,
                        GateType::Mul => l * r,
                    }
                })
                .collect();
            values[i] = out;
        }

        values
    }
}

#[derive(Clone, Debug)]
pub struct LayerProof<F: PrimeField> {
    /// One `[s(0), s(1), s(2)]` per sumcheck round.
    pub sumcheck: Vec<[F; 3]>,
    /// Claimed value of the layer-below MLE at the two sumcheck points.
    pub w_u: F,
    pub w_v: F,
    /// Evaluations of the restriction of the layer-below MLE to the line
    /// through the two sumcheck points, used to fold two claims into one.
    pub line: Vec<F>,
}

#[derive(Clone, Debug)]
pub struct GKRProof<F: PrimeField> {
    pub output: Vec<F>,
    pub layers: Vec<LayerProof<F>>,
}

/// `log2` of a power-of-two width.
pub fn num_vars(width: usize) -> usize {
    assert!(width.is_power_of_two(), "layer widths must be powers of two");
    width.trailing_zeros() as usize
}

/// Multilinear equality basis: `eq(point, index)` where `index` is read as an
/// `nbits`-bit boolean vector with the most significant bit first.
pub fn eq_index<F: PrimeField>(point: &[F], index: usize, nbits: usize) -> F {
    let mut acc = F::one();
    for j in 0..nbits {
        let bit = (index >> (nbits - 1 - j)) & 1;
        if bit == 1 {
            acc *= point[j];
        } else {
            acc *= F::one() - point[j];
        }
    }
    acc
}

/// Evaluate the multilinear extension of `table` at `point`.
pub fn eval_mle<F: PrimeField>(table: &[F], point: &[F]) -> F {
    let m = point.len();
    debug_assert_eq!(table.len(), 1 << m);
    let mut acc = F::zero();
    for (idx, value) in table.iter().enumerate() {
        if value.is_zero() {
            continue;
        }
        acc += *value * eq_index(point, idx, m);
    }
    acc
}

/// Evaluate the polynomial that passes through `evals` at nodes `0, 1, 2, ...`.
pub fn lagrange_eval<F: PrimeField>(evals: &[F], x: F) -> F {
    let n = evals.len();
    let mut result = F::zero();
    for i in 0..n {
        let xi = F::from(i as u64);
        let mut num = F::one();
        let mut den = F::one();
        for j in 0..n {
            if j != i {
                let xj = F::from(j as u64);
                num *= x - xj;
                den *= xi - xj;
            }
        }
        result += evals[i] * num * den.inverse().expect("distinct nodes");
    }
    result
}

/// A Fiat-Shamir transcript backed by SHA-256.
pub struct Transcript {
    state: [u8; 32],
}

impl Transcript {
    pub fn new(label: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(label);
        let mut state = [0u8; 32];
        state.copy_from_slice(&hasher.finalize());
        Transcript { state }
    }

    pub fn absorb<F: CanonicalSerialize>(&mut self, value: &F) {
        let mut bytes = Vec::new();
        value.serialize_compressed(&mut bytes).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(self.state);
        hasher.update(&bytes);
        self.state.copy_from_slice(&hasher.finalize());
    }

    pub fn challenge<F: PrimeField>(&mut self) -> F {
        let mut hasher = Sha256::new();
        hasher.update(self.state);
        hasher.update(b"challenge");
        let out = hasher.finalize();
        self.state.copy_from_slice(&out);
        F::from_le_bytes_mod_order(&out)
    }
}
