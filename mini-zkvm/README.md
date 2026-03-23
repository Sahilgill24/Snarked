## Mini-ZKVM 

This is a miniature ZK virtual machine, that works actually and can generate a proof. 
</br>
`/vm`
Code for the basic VM and emulated registers to generate the execution trace. 
</br>
`/lookup && /trace`
So now this is the lookup where we generate the poly's from the execution trace and Plookup. 
</br>
`/stark`
This is where the Stark stuff happens. 

## Architecture Overview

### VM Module
- **Registers**: 16 general-purpose registers (r0-r15) with configurable width
- **CPU**: 5-stage pipeline fetch-execute cycle with instruction decoding
- **Memory**: Addressable memory with stack pointer management

### Trace Module
- **Execution Trace**: Records CPU state at each cycle
- **Constraint Polynomials**: Generates constraints for proof verification
- **Merkle Commitments**: Creates polynomial commitments using Merkle trees

### STARK Module
- **Proof Generation**: Creates zero-knowledge proofs using FRI
- **Proof Verification**: Verifies STARK proofs against constraints
- **Prover Binary**: Unified interface for executing programs and generating proofs

## Usage

\`\`\`bash
cargo run --bin stark -- path/to/program.bin
\`\`\`

