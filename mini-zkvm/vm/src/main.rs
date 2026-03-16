// This is a register based VM, which will generate a trace table
// the registers are defined here.
// they will be used to define the execution's trace.

// ADD r3 r1 r2
// 0000 | 0011 | 0001 | 0010

// MOV r1 7
// 0011 | 0001| 0111 | xxxx {Challenge is there is no fixed bit size and format, and I cant simply pad or can I 0000 -> 0111, 1000 would work and represent no bit}

use vm::{Cpu, get_instruction};

fn main() {
    // Example 1: Simple MOV and ADD
    println!("=== VM Example ===\n");
    
    let program = vec![
        0x1102, // MOV r1, 2      (r1 = 2)
        0x1203, // MOV r2, 3      (r2 = 3)
        0x3012, // ADD r0, r1, r2 (r0 = r1 + r2 = 5)
    ];

    let mut cpu = Cpu::new(program);
    cpu.run();

    println!("Execution completed!");
    println!("Final register state:");
    let regs = cpu.registers.get_all_registers();
    for (i, &val) in regs.iter().enumerate() {
        println!("  r{}: {}", i, val);
    }

    println!("\nExecution trace:");
    for (i, row) in cpu.get_trace().iter().enumerate() {
        println!(
            "  Step {}: PC={}, Instruction=0x{:04X}",
            i, row.pc, row.instruction
        );
    }

    // Test instruction parsing
    println!("\n=== Instruction Decoding ===");
    let test_instruction = 0x3012;
    let opcode = get_instruction(test_instruction);
    println!("Instruction 0x{:04X}: {:?}", test_instruction, opcode);
}

// CLI and program loading March 2026
