# Snarked

My attempt at building SNARK- and STARK-based systems from scratch to learn how
they actually work: a small zero-knowledge virtual machine, and hand-written
implementations of a few proof systems. It's a learning repo, so some pieces are
complete and runnable while others are still skeletons — the status of each is
spelled out below.

## mini-zkvm

A 16-bit register machine with an assembler and a (deliberately simplified)
trace-to-proof pipeline. You write a program, the VM runs it and records a
per-cycle execution trace, and the later stages turn that trace into constraint
polynomials and a toy proof.

Status: **working.** Builds clean, runs, and has tests.

```bash
cd mini-zkvm
cargo run --bin vm -- examples/sum.asm   # assemble and run a program
cargo run --bin stark                    # trace + constraints + proof over demo programs
cargo test
```

See [`mini-zkvm/README.md`](mini-zkvm/README.md) for the instruction set and the
assembly syntax.

## snarks

Each of these is a standalone crate under `snarks/`.

### succinct_gkr_protocol

A real implementation of the GKR interactive proof for layered arithmetic
circuits, made non-interactive with Fiat-Shamir. The prover shows that a circuit
maps a public input to a claimed output; each layer's claim is reduced to the
layer below with the sumcheck protocol, so the verifier's work scales with the
circuit's width and depth rather than its size.

Status: **working.** Honest proofs verify; tampered proofs and wrong inputs are
rejected.

```bash
cd snarks/succinct_gkr_protocol
cargo test
cargo run --example demo
```

### groth16

A simplified, pairing-based Groth16 skeleton (QAP types, trusted setup, proving
and verifying keys). Status: **compiles**, but it's a partial scaffold rather
than a complete, sound prover.

### plonk

Split into `plonk/core`, `plonk/protocol`, and `plonk/compiler`. Status: `core`
and `compiler` **compile**; `protocol` is still a work in progress (its pairing
setup doesn't type-check yet). Treat this as scaffolding.

## Layout

```
mini-zkvm/                  register VM + trace + toy STARK pipeline
snarks/
  succinct_gkr_protocol/    working GKR (sumcheck + Fiat-Shamir)
  groth16/                  Groth16 scaffold
  plonk/{core,protocol,compiler}
```

## Building

There's no top-level workspace — each crate builds on its own, so a
work-in-progress crate never blocks the ones that work. `cd` into a crate and
run `cargo build` / `cargo test` / `cargo run`.
