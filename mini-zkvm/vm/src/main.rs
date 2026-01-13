// Register-based VM. Runs either a built-in demo or an assembly file
// passed on the command line, and prints the final register state and
// execution trace.
//
//   ADD r3, r1, r2  ->  0000 0011 0001 0010  (0x3312)
//   MOV r1, 7       ->  0001 0001 0000 0111  (0x1107)

use std::process;

use vm::{assemble, get_instruction, Cpu};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let program = match args.get(1) {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(source) => match assemble(&source) {
                Ok(program) => program,
                Err(e) => {
                    eprintln!("assembly error: {e}");
                    process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("could not read `{path}`: {e}");
                process::exit(1);
            }
        },
        None => {
            println!("=== VM Demo (no program file given) ===\n");
            vec![
                0x1102, // MOV r1, 2
                0x1203, // MOV r2, 3
                0x3012, // ADD r0, r1, r2  -> r0 = 5
            ]
        }
    };

    let mut cpu = Cpu::new(program);
    cpu.run();

    println!("Execution completed!");
    println!("Final register state:");
    for (i, val) in cpu.registers.get_all_registers().iter().enumerate() {
        println!("  r{i}: {val}");
    }

    println!("\nExecution trace:");
    for (i, row) in cpu.get_trace().iter().enumerate() {
        println!(
            "  Step {i}: PC={}, Instruction=0x{:04X} ({:?})",
            row.pc,
            row.instruction,
            get_instruction(row.instruction)
        );
    }
}
