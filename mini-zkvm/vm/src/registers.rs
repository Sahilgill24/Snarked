use serde::{Deserialize, Serialize};

pub const NUM_REGISTERS: usize = 8;

#[derive(Debug, Default, Clone)]
pub struct Register {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
}

impl Register {
    pub fn new() -> Register {
        Register::default()
    }

    pub fn get_reg_from_idx(&self, idx: u16) -> Option<u16> {
        match idx {
            0 => Some(self.r0),
            1 => Some(self.r1),
            2 => Some(self.r2),
            3 => Some(self.r3),
            4 => Some(self.r4),
            5 => Some(self.r5),
            6 => Some(self.r6),
            7 => Some(self.r7),
            _ => None,
        }
    }

    pub fn set_reg(&mut self, idx: u16, val: u16) {
        match idx {
            0 => self.r0 = val,
            1 => self.r1 = val,
            2 => self.r2 = val,
            3 => self.r3 = val,
            4 => self.r4 = val,
            5 => self.r5 = val,
            6 => self.r6 = val,
            7 => self.r7 = val,
            _ => {}
        }
    }

    pub fn get_all_registers(&self) -> [u16; NUM_REGISTERS] {
        [
            self.r0, self.r1, self.r2, self.r3, self.r4, self.r5, self.r6, self.r7,
        ]
    }

    pub fn to_trace_row(&self, pc: u32, instruction: u16) -> TraceRow {
        TraceRow {
            pc,
            instruction,
            registers: self.get_all_registers(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRow {
    pub pc: u32,
    pub instruction: u16,
    pub registers: [u16; NUM_REGISTERS],
}

/// Destination register, encoded in bits 11..8.
pub fn decode_rd(instruction: u16) -> u16 {
    (instruction >> 8) & 0xF
}

/// First source register, encoded in bits 7..4.
pub fn decode_rs(instruction: u16) -> u16 {
    (instruction >> 4) & 0xF
}

/// Second source register, encoded in bits 3..0.
pub fn decode_rt(instruction: u16) -> u16 {
    instruction & 0xF
}
