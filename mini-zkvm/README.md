# mini-zkvm

A small register machine written to understand how execution traces turn into
STARK-style proofs. You write a program, the VM runs it and records what every
register held on each cycle, and the later stages take that trace and produce a
(deliberately simplified) proof about it.

It's a learning project, not a sound proof system. The "proof" stage stands in
for the real cryptography so the data flow is easy to follow end to end.

## Layout

The workspace has three crates:

- `vm` — the machine itself: registers, the instruction set, the fetch/execute
  loop, and a tiny assembler.
- `trace` — takes a finished run and builds the execution trace, the constraint
  polynomials, and a toy proof over them.
- `stark` — a binary that wires the two together, runs a few example programs,
  and prints/serialises the results.

## The machine

- 8 registers, `r0`–`r7`, each 16 bits.
- Instructions are a single 16-bit word. The top 4 bits are the opcode; the
  rest encode operands.
- One flat memory (the program is loaded into it; `LOAD`/`STORE` read and write
  it). Execution stops when the program counter runs past the loaded program.

### Instruction set

| Opcode | Mnemonic | Form               | Meaning                          |
|-------:|----------|--------------------|----------------------------------|
| 0x0    | `NOP`    | `NOP`              | do nothing                       |
| 0x1    | `MOV`    | `MOV rd, imm`      | `rd = imm` (8-bit immediate)     |
| 0x2    | `MOVR`   | `MOVR rd, rs`      | `rd = rs`                        |
| 0x3    | `ADD`    | `ADD rd, rs, rt`   | `rd = rs + rt` (wrapping)        |
| 0x4    | `SUB`    | `SUB rd, rs, rt`   | `rd = rs - rt` (wrapping)        |
| 0x5    | `MUL`    | `MUL rd, rs, rt`   | `rd = rs * rt` (wrapping)        |
| 0x6    | `DIV`    | `DIV rd, rs, rt`   | `rd = rs / rt` (0 on divide-by-0)|
| 0x7    | `AND`    | `AND rd, rs, rt`   | `rd = rs & rt`                   |
| 0x8    | `OR`     | `OR rd, rs, rt`    | `rd = rs | rt`                   |
| 0x9    | `XOR`    | `XOR rd, rs, rt`   | `rd = rs ^ rt`                   |
| 0xA    | `NOT`    | `NOT rd, rs`       | `rd = !rs`                       |
| 0xB    | `JMP`    | `JMP addr`         | `pc = addr` (12-bit)             |
| 0xC    | `JEQ`    | `JEQ rs, rt, addr` | jump to `addr` if `rs == rt`     |
| 0xD    | `JNE`    | `JNE rs, rt, addr` | jump to `addr` if `rs != rt`     |
| 0xE    | `LOAD`   | `LOAD rd, addr`    | `rd = memory[addr]` (8-bit addr) |
| 0xF    | `STORE`  | `STORE rs, addr`   | `memory[addr] = rs` (8-bit addr) |

Registers sit in bits `[11:8]` (`rd`), `[7:4]` (`rs`), and `[3:0]` (`rt`).
So `ADD r0, r1, r2` is `0x3012`.

## Assembly

Programs are one instruction per line. `;` starts a comment, immediates are
decimal or `0x`-prefixed hex, and blank lines are ignored. See
[`examples/sum.asm`](examples/sum.asm):

```asm
; compute (2 + 3) * 4 and leave the result in r0
MOV  r1, 2
MOV  r2, 3
ADD  r0, r1, r2
MOV  r3, 4
MUL  r0, r0, r3
STORE r0, 64
LOAD r4, 64
```

## Running it

Run an assembly file through the VM and print the final registers plus the trace:

```bash
cargo run --bin vm -- examples/sum.asm
```

With no file it runs a built-in demo instead.

Run the STARK stage over a few hard-coded programs (trace, constraints, proof,
and JSON exports):

```bash
cargo run --bin stark
```

Tests:

```bash
cargo test
```

## What the trace and proof stages actually do

`trace` records one row per cycle (program counter, instruction, all eight
registers), then flattens each register/instruction/pc column into coefficient
vectors it calls constraint polynomials. The proof is a rolling hash of the
trace plus a handful of polynomial evaluations at fixed points.

None of that is a real STARK — there's no low-degree extension, no FRI, no
soundness. It's a scaffold that has the same shape as the real pipeline
(trace → constraints → commitment → evaluation) so each piece is legible before
the cryptography goes in.
