use crate::instructions::{get_instruction, InstructionSet};
use crate::registers::{Register, TraceRow};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub registers: Register,
    pub pc: u32,
    pub sp: u32,
    pub memory: Vec<u16>,
    pub running: bool,
    pub trace: Vec<TraceRow>,
}

impl Cpu {
    pub fn new(program: Vec<u16>) -> Self {
        Cpu {
            registers: Register::new(),
            pc: 0,
            sp: 1024,
            memory: program,
            running: true,
            trace: Vec::new(),
        }
    }

    pub fn fetch(&self) -> Option<u16> {
        self.memory.get(self.pc as usize).copied()
    }

    pub fn execute(&mut self, instruction: u16) -> bool {
        let opcode = get_instruction(instruction);

        // Record trace before execution
        self.trace
            .push(self.registers.to_trace_row(self.pc, instruction));

        match opcode {
            InstructionSet::NOP => {
                self.pc += 1;
            }
            InstructionSet::MOV => {
                // MOV Rd, imm: bits[15:12]=0001, bits[11:8]=Rd, bits[7:0]=imm
                let rd = (instruction >> 8) & 0xF;
                let imm = instruction & 0xFF;
                self.registers.set_reg(rd as u16, imm as u16);
                self.pc += 1;
            }
            InstructionSet::MOVR => {
                // MOVR Rd, Rs: bits[15:12]=0010, bits[11:8]=Rd, bits[7:4]=Rs
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                if let Some(val) = self.registers.get_reg_from_idx(rs as u16) {
                    self.registers.set_reg(rd as u16, val);
                }
                self.pc += 1;
            }
            InstructionSet::ADD => {
                // ADD Rd, Rs, Rt: bits[15:12]=0011, bits[11:8]=Rd, bits[7:4]=Rs, bits[3:0]=Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val.wrapping_add(rt_val);
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::SUB => {
                // SUB Rd, Rs, Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val.wrapping_sub(rt_val);
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::MUL => {
                // MUL Rd, Rs, Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val.wrapping_mul(rt_val);
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::AND => {
                // AND Rd, Rs, Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val & rt_val;
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::OR => {
                // OR Rd, Rs, Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val | rt_val;
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::XOR => {
                // XOR Rd, Rs, Rt
                let rd = (instruction >> 8) & 0xF;
                let rs = (instruction >> 4) & 0xF;
                let rt = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    let result = rs_val ^ rt_val;
                    self.registers.set_reg(rd as u16, result);
                }
                self.pc += 1;
            }
            InstructionSet::JMP => {
                // JMP addr: bits[15:12]=1011, bits[11:0]=addr
                let addr = instruction & 0xFFF;
                self.pc = addr as u32;
            }
            InstructionSet::JEQ => {
                // JEQ Rs, Rt, addr: Jump if equal
                let rs = (instruction >> 8) & 0xF;
                let rt = (instruction >> 4) & 0xF;
                let addr = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    if rs_val == rt_val {
                        self.pc = addr as u32;
                    } else {
                        self.pc += 1;
                    }
                }
            }
            InstructionSet::JNE => {
                // JNE Rs, Rt, addr: Jump if not equal
                let rs = (instruction >> 8) & 0xF;
                let rt = (instruction >> 4) & 0xF;
                let addr = instruction & 0xF;
                if let (Some(rs_val), Some(rt_val)) = (
                    self.registers.get_reg_from_idx(rs as u16),
                    self.registers.get_reg_from_idx(rt as u16),
                ) {
                    if rs_val != rt_val {
                        self.pc = addr as u32;
                    } else {
                        self.pc += 1;
                    }
                }
            }
            _ => {
                self.pc += 1;
            }
        }

        // Continue execution if pc is still within program bounds
        self.pc < self.memory.len() as u32
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.fetch() {
            if !self.execute(instruction) {
                break;
            }
        }
    }

    pub fn get_trace(&self) -> Vec<TraceRow> {
        self.trace.clone()
    }
}
