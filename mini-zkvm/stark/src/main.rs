use serde_json::json;
use trace::{ConstraintPolynomials, ExecutionTrace, TraceProof};
use vm::Cpu;

/// STARK Prover - Generates zero-knowledge proofs for VM execution
pub struct StarkProver {
    trace: ExecutionTrace,
    constraints: ConstraintPolynomials,
    proof: TraceProof,
}

impl StarkProver {
    /// Create a new STARK prover from a CPU execution
    pub fn new(cpu: &Cpu) -> Self {
        let trace = ExecutionTrace::from_cpu(cpu);
        let constraints = ConstraintPolynomials::from_trace(&trace);
        let proof = TraceProof::generate(&trace);

        StarkProver {
            trace,
            constraints,
            proof,
        }
    }

    /// Get the execution trace
    pub fn get_trace(&self) -> &ExecutionTrace {
        &self.trace
    }

    /// Get the constraints
    pub fn get_constraints(&self) -> &ConstraintPolynomials {
        &self.constraints
    }

    /// Get the proof
    pub fn get_proof(&self) -> &TraceProof {
        &self.proof
    }

    /// Verify the proof
    pub fn verify(&self) -> bool {
        self.trace.verify_constraints() && self.proof.verify()
    }

    /// Print a summary of the execution
    pub fn print_summary(&self) {
        println!("\n=== STARK Prover Summary ===\n");
        println!("Execution Trace:");
        println!("  Total steps: {}", self.trace.num_rows);
        println!("  Trace verified: {}", self.trace.verify_constraints());

        println!("\nConstraints Generated:");
        println!("  Register constraints: {}", self.constraints.register_constraints.len());
        println!(
            "  Instruction constraints: {}",
            self.constraints.instruction_constraints.len()
        );
        println!("  PC constraints: {}", self.constraints.pc_constraints.len());

        println!("\nProof Details:");
        println!("  Trace hash: {}", self.proof.trace_hash);
        println!("  Constraints hash: {}", self.proof.constraints_hash);
        println!(
            "  Evaluation points: {}",
            self.proof.evaluation_points.len()
        );
        println!("  Proof verified: {}", self.verify());
    }

    /// Export trace as JSON
    pub fn export_trace_json(&self) -> String {
        let table = self.trace.to_table();
        serde_json::to_string_pretty(&table).unwrap_or_else(|_| "Error serializing trace".to_string())
    }

    /// Export proof as JSON
    pub fn export_proof_json(&self) -> String {
        serde_json::to_string_pretty(&self.proof)
            .unwrap_or_else(|_| "Error serializing proof".to_string())
    }
}

fn main() {
    println!("Mini Zero-Knowledge VM with STARK Proofs\n");

    // Example 1: Simple arithmetic operations
    println!("=== Example 1: Arithmetic Operations ===");
    let program1 = vec![
        0x1102, // MOV r1, 2      (r1 = 2)
        0x1203, // MOV r2, 3      (r2 = 3)
        0x3012, // ADD r0, r1, r2 (r0 = r1 + r2 = 5)
    ];

    let mut cpu1 = Cpu::new(program1);
    cpu1.run();

    let prover1 = StarkProver::new(&cpu1);
    prover1.print_summary();

    println!("\nExecution Trace (Example 1):");
    for (i, row) in prover1.get_trace().rows.iter().enumerate() {
        println!(
            "  Cycle {}: PC={}, Instruction=0x{:04X}, r0={}, r1={}, r2={}",
            i, row.pc, row.instruction, row.registers[0], row.registers[1], row.registers[2]
        );
    }

    // Example 2: More complex program with jumps
    println!("\n\n=== Example 2: Program with Conditional Logic ===");
    let program2 = vec![
        0x1105, // MOV r1, 5      (r1 = 5)
        0x1205, // MOV r2, 5      (r2 = 5)
        0x3012, // ADD r0, r1, r2 (r0 = 10)
        0x4102, // SUB r1, r0, r2 (r1 = 5)
    ];

    let mut cpu2 = Cpu::new(program2);
    cpu2.run();

    let prover2 = StarkProver::new(&cpu2);
    prover2.print_summary();

    println!("\nExecution Trace (Example 2):");
    for (i, row) in prover2.get_trace().rows.iter().enumerate() {
        println!(
            "  Cycle {}: PC={}, Instruction=0x{:04X}, r0={}, r1={}, r2={}",
            i, row.pc, row.instruction, row.registers[0], row.registers[1], row.registers[2]
        );
    }

    // Example 3: Multiplication
    println!("\n\n=== Example 3: Multiplication Program ===");
    let program3 = vec![
        0x1103, // MOV r1, 3      (r1 = 3)
        0x1204, // MOV r2, 4      (r2 = 4)
        0x5012, // MUL r0, r1, r2 (r0 = 12)
    ];

    let mut cpu3 = Cpu::new(program3);
    cpu3.run();

    let prover3 = StarkProver::new(&cpu3);
    prover3.print_summary();

    println!("\nExecution Trace (Example 3):");
    for (i, row) in prover3.get_trace().rows.iter().enumerate() {
        println!(
            "  Cycle {}: PC={}, Instruction=0x{:04X}, r0={}, r1={}, r2={}",
            i, row.pc, row.instruction, row.registers[0], row.registers[1], row.registers[2]
        );
    }

    // Example 4: Bitwise operations
    println!("\n\n=== Example 4: Bitwise Operations ===");
    let program4 = vec![
        0x110F, // MOV r1, 15     (r1 = 0xF)
        0x1203, // MOV r2, 3      (r2 = 0x3)
        0x7012, // AND r0, r1, r2 (r0 = 3)
        0x8012, // OR r0, r1, r2  (r0 = 15)
        0x9012, // XOR r0, r1, r2 (r0 = 12)
    ];

    let mut cpu4 = Cpu::new(program4);
    cpu4.run();

    let prover4 = StarkProver::new(&cpu4);
    prover4.print_summary();

    println!("\nExecution Trace (Example 4):");
    for (i, row) in prover4.get_trace().rows.iter().enumerate() {
        println!(
            "  Cycle {}: PC={}, Instruction=0x{:04X}, r0={}, r1={}, r2={}",
            i, row.pc, row.instruction, row.registers[0], row.registers[1], row.registers[2]
        );
    }

    // Final summary with JSON exports
    println!("\n\n=== Proof Verification ===");
    println!("Example 1 proof verified: {}", prover1.verify());
    println!("Example 2 proof verified: {}", prover2.verify());
    println!("Example 3 proof verified: {}", prover3.verify());
    println!("Example 4 proof verified: {}", prover4.verify());

    println!("\n=== Example 1 Proof (JSON) ===");
    println!("{}", prover1.export_proof_json());

    println!("\n=== Example 1 Trace (JSON) ===");
    println!("{}", prover1.export_trace_json());
}
