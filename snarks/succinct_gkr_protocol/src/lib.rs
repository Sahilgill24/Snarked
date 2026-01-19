//! A working implementation of the GKR interactive proof for layered
//! arithmetic circuits, made non-interactive with Fiat-Shamir.
//!
//! The prover convinces the verifier that a circuit evaluates a public input to
//! a claimed output. Each layer's claim is reduced to a claim about the layer
//! below via the sumcheck protocol; the verifier only ever materialises the
//! input, so its work is proportional to the circuit's width and depth rather
//! than its size.

pub mod interfaces;
pub mod primitives;

use ark_ff::PrimeField;

use interfaces::{GKRProverInterface, GKRVerifierInterface};
use primitives::{
    eq_index, eval_mle, lagrange_eval, num_vars, Circuit, GKRProof, GateType, Layer, LayerProof,
    Transcript,
};

pub struct SuccinctGKR;

impl<F: PrimeField> GKRProverInterface<F> for SuccinctGKR {
    fn prove(circuit: &Circuit, input: &[F]) -> GKRProof<F> {
        prove(circuit, input)
    }
}

impl<F: PrimeField> GKRVerifierInterface<F> for SuccinctGKR {
    fn verify(circuit: &Circuit, proof: &GKRProof<F>, input: &[F]) -> bool {
        verify(circuit, proof, input)
    }
}

/// Width of the wire values at index `i` (`layers.len()` is the input layer).
fn layer_width(circuit: &Circuit, input_len: usize, i: usize) -> usize {
    if i < circuit.layers.len() {
        circuit.layers[i].gates.len()
    } else {
        input_len
    }
}

/// Wiring-predicate tables `add(r, x, y)` and `mul(r, x, y)` for a fixed output
/// point `r`, laid out over `(x, y)` with `x` in the high bits.
fn wiring_tables<F: PrimeField>(layer: &Layer, r: &[F], ki: usize, k1: usize) -> (Vec<F>, Vec<F>) {
    let size = 1usize << (2 * k1);
    let mut add = vec![F::zero(); size];
    let mut mul = vec![F::zero(); size];
    for (z, gate) in layer.gates.iter().enumerate() {
        let weight = eq_index(r, z, ki);
        let idx = gate.left * (1 << k1) + gate.right;
        match gate.gate_type {
            GateType::Add => add[idx] += weight,
            GateType::Mul => mul[idx] += weight,
        }
    }
    (add, mul)
}

/// Verifier-side evaluation of the wiring predicates at `(r, us, vs)`.
fn wiring_eval<F: PrimeField>(
    layer: &Layer,
    r: &[F],
    us: &[F],
    vs: &[F],
    ki: usize,
    k1: usize,
) -> (F, F) {
    let mut add = F::zero();
    let mut mul = F::zero();
    for (z, gate) in layer.gates.iter().enumerate() {
        let weight =
            eq_index(r, z, ki) * eq_index(us, gate.left, k1) * eq_index(vs, gate.right, k1);
        match gate.gate_type {
            GateType::Add => add += weight,
            GateType::Mul => mul += weight,
        }
    }
    (add, mul)
}

/// Sumcheck prover for a degree-2 polynomial `g` over `n` variables. Returns the
/// per-round evaluations at `t = 0, 1, 2` and the challenges drawn.
fn sumcheck_prove<F: PrimeField>(
    n: usize,
    transcript: &mut Transcript,
    g: impl Fn(&[F]) -> F,
) -> (Vec<[F; 3]>, Vec<F>) {
    let mut challenges: Vec<F> = Vec::new();
    let mut polys = Vec::new();

    for round in 0..n {
        let free = n - round - 1;
        let mut evals = [F::zero(); 3];
        for (ti, eval) in evals.iter_mut().enumerate() {
            let t = F::from(ti as u64);
            let mut acc = F::zero();
            for mask in 0..(1usize << free) {
                let mut point = challenges.clone();
                point.push(t);
                for j in 0..free {
                    let bit = (mask >> (free - 1 - j)) & 1;
                    point.push(F::from(bit as u64));
                }
                acc += g(&point);
            }
            *eval = acc;
        }
        for e in &evals {
            transcript.absorb(e);
        }
        challenges.push(transcript.challenge());
        polys.push(evals);
    }

    (polys, challenges)
}

/// Sumcheck verifier. Returns the challenges and the final reduced claim (the
/// value the caller must check equals `g(challenges)`), or `None` on failure.
fn sumcheck_verify<F: PrimeField>(
    n: usize,
    mut claim: F,
    polys: &[[F; 3]],
    transcript: &mut Transcript,
) -> Option<(Vec<F>, F)> {
    if polys.len() != n {
        return None;
    }
    let mut challenges = Vec::new();
    for round in polys.iter().take(n) {
        if round[0] + round[1] != claim {
            return None;
        }
        for e in round {
            transcript.absorb(e);
        }
        let c = transcript.challenge();
        claim = lagrange_eval(round, c);
        challenges.push(c);
    }
    Some((challenges, claim))
}

pub fn prove<F: PrimeField>(circuit: &Circuit, input: &[F]) -> GKRProof<F> {
    let values = circuit.evaluate(input);
    let depth = circuit.layers.len();

    let mut transcript = Transcript::new(b"succinct-gkr");
    for v in &values[0] {
        transcript.absorb(v);
    }
    for v in input {
        transcript.absorb(v);
    }

    let mut ki = num_vars(values[0].len());
    let mut r: Vec<F> = (0..ki).map(|_| transcript.challenge()).collect();

    let mut layer_proofs = Vec::with_capacity(depth);
    for i in 0..depth {
        let below = &values[i + 1];
        let k1 = num_vars(below.len());
        let (add, mul) = wiring_tables::<F>(&circuit.layers[i], &r, ki, k1);

        let g = |point: &[F]| -> F {
            let (u, v) = point.split_at(k1);
            let a = eval_mle(&add, point);
            let m = eval_mle(&mul, point);
            let wu = eval_mle(below, u);
            let wv = eval_mle(below, v);
            a * (wu + wv) + m * wu * wv
        };

        let (sumcheck, challenges) = sumcheck_prove(2 * k1, &mut transcript, g);
        let (us, vs) = challenges.split_at(k1);
        let w_u = eval_mle(below, us);
        let w_v = eval_mle(below, vs);
        transcript.absorb(&w_u);
        transcript.absorb(&w_v);

        let line = line_evals(below, us, vs, k1);
        for e in &line {
            transcript.absorb(e);
        }
        let t_star: F = transcript.challenge();
        r = fold_line(us, vs, t_star);
        ki = k1;

        layer_proofs.push(LayerProof { sumcheck, w_u, w_v, line });
    }

    GKRProof { output: values[0].clone(), layers: layer_proofs }
}

pub fn verify<F: PrimeField>(circuit: &Circuit, proof: &GKRProof<F>, input: &[F]) -> bool {
    let depth = circuit.layers.len();
    if proof.layers.len() != depth || proof.output.len() != layer_width(circuit, input.len(), 0) {
        return false;
    }

    let mut transcript = Transcript::new(b"succinct-gkr");
    for v in &proof.output {
        transcript.absorb(v);
    }
    for v in input {
        transcript.absorb(v);
    }

    let mut ki = num_vars(proof.output.len());
    let mut r: Vec<F> = (0..ki).map(|_| transcript.challenge()).collect();
    let mut m = eval_mle(&proof.output, &r);

    for i in 0..depth {
        let lp = &proof.layers[i];
        let k1 = num_vars(layer_width(circuit, input.len(), i + 1));

        let (challenges, final_claim) =
            match sumcheck_verify(2 * k1, m, &lp.sumcheck, &mut transcript) {
                Some(x) => x,
                None => return false,
            };
        let (us, vs) = challenges.split_at(k1);

        let (a_eval, m_eval) = wiring_eval(&circuit.layers[i], &r, us, vs, ki, k1);
        let g_val = a_eval * (lp.w_u + lp.w_v) + m_eval * lp.w_u * lp.w_v;
        if g_val != final_claim {
            return false;
        }
        transcript.absorb(&lp.w_u);
        transcript.absorb(&lp.w_v);

        if lp.line.len() != k1 + 1 || lp.line[0] != lp.w_u {
            return false;
        }
        if k1 >= 1 && lp.line[1] != lp.w_v {
            return false;
        }
        for e in &lp.line {
            transcript.absorb(e);
        }
        let t_star: F = transcript.challenge();
        r = fold_line(us, vs, t_star);
        m = lagrange_eval(&lp.line, t_star);
        ki = k1;
    }

    eval_mle(input, &r) == m
}

/// Evaluations of `W(line(t))` at `t = 0, 1, ..., k1`, where `line` runs from
/// `us` to `vs`.
fn line_evals<F: PrimeField>(table: &[F], us: &[F], vs: &[F], k1: usize) -> Vec<F> {
    (0..=k1)
        .map(|node| {
            let point = fold_line(us, vs, F::from(node as u64));
            eval_mle(table, &point)
        })
        .collect()
}

fn fold_line<F: PrimeField>(us: &[F], vs: &[F], t: F) -> Vec<F> {
    us.iter().zip(vs).map(|(a, b)| *a + t * (*b - *a)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use primitives::Gate;

    // Output layer (width 2) over an intermediate layer (width 2) over a
    // width-4 input:
    //   t0 = in0 + in1
    //   t1 = in2 * in3
    //   out0 = t0 + t1
    //   out1 = t0 * t1
    fn sample_circuit() -> Circuit {
        let output = Layer::new(vec![Gate::add(0, 1), Gate::mul(0, 1)]);
        let middle = Layer::new(vec![Gate::add(0, 1), Gate::mul(2, 3)]);
        Circuit::new(vec![output, middle])
    }

    #[test]
    fn evaluates_circuit() {
        let input = [Fr::from(2u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];
        let values = sample_circuit().evaluate(&input);
        // t0 = 5, t1 = 20 -> out = [25, 100]
        assert_eq!(values[0], vec![Fr::from(25u64), Fr::from(100u64)]);
    }

    #[test]
    fn honest_proof_verifies() {
        let circuit = sample_circuit();
        let input = [Fr::from(2u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];
        let proof = prove(&circuit, &input);
        assert!(verify(&circuit, &proof, &input));
    }

    #[test]
    fn tampered_output_rejected() {
        let circuit = sample_circuit();
        let input = [Fr::from(2u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];
        let mut proof = prove(&circuit, &input);
        proof.output[0] += Fr::from(1u64);
        assert!(!verify(&circuit, &proof, &input));
    }

    #[test]
    fn wrong_input_rejected() {
        let circuit = sample_circuit();
        let input = [Fr::from(2u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];
        let proof = prove(&circuit, &input);
        let wrong = [Fr::from(9u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];
        assert!(!verify(&circuit, &proof, &wrong));
    }

    #[test]
    fn single_output_circuit() {
        // Width-1 output layer exercises the k = 0 path.
        let output = Layer::new(vec![Gate::add(0, 1)]);
        let middle = Layer::new(vec![Gate::mul(0, 1), Gate::add(2, 3)]);
        let circuit = Circuit::new(vec![output, middle]);
        let input = [Fr::from(6u64), Fr::from(7u64), Fr::from(8u64), Fr::from(9u64)];
        let proof = prove(&circuit, &input);
        assert!(verify(&circuit, &proof, &input));
        assert_eq!(proof.output, vec![Fr::from(6u64 * 7 + 8 + 9)]);
    }
}
