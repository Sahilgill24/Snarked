#[derive(Debug)]
pub enum InstructionSet {
    NOP,
    MOV,
    MOVR,
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    XOR,
    NOT,
    JMP,
    JEQ,
    JNE,
    LOAD,
    STORE,
}

impl InstructionSet {}

// 16-bit virtual Machine Assumption
pub fn get_instruction(instruction: u16) -> InstructionSet {
    // the left most 4 bits represent the Instruction set
    match instruction >> 12 {
        0 => InstructionSet::NOP,
        1 => InstructionSet::MOV,
        2 => InstructionSet::MOVR,
        3 => InstructionSet::ADD,
        4 => InstructionSet::SUB,
        5 => InstructionSet::MUL,
        6 => InstructionSet::DIV,
        7 => InstructionSet::AND,
        8 => InstructionSet::OR,
        9 => InstructionSet::XOR,
        10 => InstructionSet::NOT,
        11 => InstructionSet::JMP,
        12 => InstructionSet::JEQ,
        13 => InstructionSet::JNE,
        14 => InstructionSet::LOAD,
        15 => InstructionSet::STORE,
        _ => InstructionSet::NOP,
    }
}



pub fn instruction_parser(input_string: &str) {
    let mut iter = input_string.split_ascii_whitespace();
}

// Instructions documentation added in Jan 2026
// This describes the complete instruction set for the mini-zkVM
