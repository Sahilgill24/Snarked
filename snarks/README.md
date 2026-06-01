# Zero-Knowledge Proof Systems

Simplified implementations of three major ZK-SNARK systems.

## Groth16

**What it is:** A pairing-based zk-SNARK with the smallest proof size (3 group elements).

**How it works:**
- Converts circuits to Quadratic Arithmetic Programs (QAP)
- Requires trusted setup ceremony (circuit-specific)
- Prover creates proof using QAP + witness
- Verifier checks single pairing equation

**Tradeoffs:**
- Tiny proofs (~200 bytes)
- Fast verification
- Needs trusted setup per circuit
- Not universal

## PLONK

**What it is:** A universal zk-SNARK using polynomial commitments and permutation arguments.

**How it works:**
- Represents circuits as gate constraints + copy constraints
- Uses KZG commitments for polynomials
- Prover executes 5 rounds (commits to wires, permutation, quotient)
- Verifier checks polynomial identities via pairings

**Tradeoffs:**
- Universal trusted setup (one setup for all circuits)
- Custom gates possible
- Flexible circuit design
- Larger proofs than Groth16
- More complex prover

## Succinct GKR

**What it is:** Interactive proof for layered arithmetic circuits using sumcheck protocol.

**How it works:**
- Circuit evaluated layer-by-layer
- Each layer verified via sumcheck over multilinear polynomials
- Fiat-Shamir for non-interactivity
- Final layer verified with polynomial commitment

**Tradeoffs:**
- No trusted setup
- Transparent
- Efficient for structured circuits
- Proof size grows with circuit depth
- Limited to layered circuits

## Directory Structure

```
groth16/          - Groth16 implementation
plonk/            - PLONK (compiler, core, protocol)
succinct_gkr/     - GKR with sumcheck protocol
```

## Usage

Each system provides `prove()` and `verify()` functions:
- Groth16: `protocol::Groth16::prove/verify`
- PLONK: `prover::Prover::prove` + `verifier::Verifier::verify`
- GKR: `SuccinctGKR::prove/verify`

