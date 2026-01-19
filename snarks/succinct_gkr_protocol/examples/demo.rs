use ark_bn254::Fr;
use succinct_gkr_protocol::primitives::{Circuit, Gate, Layer};
use succinct_gkr_protocol::{prove, verify};

fn main() {
    // out0 = (in0 + in1) + (in2 * in3)
    // out1 = (in0 + in1) * (in2 * in3)
    let circuit = Circuit::new(vec![
        Layer::new(vec![Gate::add(0, 1), Gate::mul(0, 1)]),
        Layer::new(vec![Gate::add(0, 1), Gate::mul(2, 3)]),
    ]);

    let input = [Fr::from(2u64), Fr::from(3u64), Fr::from(4u64), Fr::from(5u64)];

    let proof = prove(&circuit, &input);
    let ok = verify(&circuit, &proof, &input);

    println!("input:  {:?}", to_u64(&input));
    println!("output: {:?}", to_u64(&proof.output));
    println!("layers in proof: {}", proof.layers.len());
    println!("verified: {ok}");

    let mut forged = proof.clone();
    forged.output[0] += Fr::from(1u64);
    println!("verified after tampering output: {}", verify(&circuit, &forged, &input));
}

fn to_u64(xs: &[Fr]) -> Vec<String> {
    xs.iter().map(|x| x.to_string()).collect()
}
