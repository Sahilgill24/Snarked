#[derive(Debug)]
pub enum Registers {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
}
#[derive(Debug, Default)]
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
        Register {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
        }
    }

    pub fn get_reg_from_idx(&mut self, idx: u16) -> Option<u16> {
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
            _ => {} // ignore invalid register indices
        }
    }
    
    pub fn get_all_registers(&self) -> [u16; 8] {
        [self.r0, self.r1, self.r2, self.r3, self.r4, self.r5, self.r6, self.r7]
    }
    
    pub fn to_trace_row(&self, pc: u32, instruction: u16) -> TraceRow {
        TraceRow {
            pc,
            instruction,
            registers: self.get_all_registers(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceRow {
    pub pc: u32,
    pub instruction: u16,
    pub registers: [u16; 8],
}
}

// 0x01E0 for bit 5-8
// 0x1E00 for bit 9-12
// pub fn get_last_register(instruction: u16) -> Option<Registers> {
//     // these are the last 4 bits
//     match instruction & 0xF {
//         0 => Some(Registers::r0),
//         1 => Some(Registers::r1),
//         2 => Some(Registers::r2),
//         3 => Some(Registers::r3),
//         4 => Some(Registers::r4),
//         5 => Some(Registers::r5),
//         6 => Some(Registers::r6),
//         7 => Some(Registers::r7),
//         _ => None,
//     }
// }

// pub fn get_first_register(instruction: u16) -> Option<Registers> {
//     match instruction & 0x01E0 {
//         0 => Some(Registers::r0),
//         1 => Some(Registers::r1),
//         2 => Some(Registers::r2),
//         3 => Some(Registers::r3),
//         4 => Some(Registers::r4),
//         5 => Some(Registers::r5),
//         6 => Some(Registers::r6),
//         7 => Some(Registers::r7),
//         _ => None,
//     }
// }

// pub fn get_dr_register(instruction: u16) -> Option<Registers> {
//     match instruction & 0x1E00 {
//         0 => Some(Registers::r0),
//         1 => Some(Registers::r1),
//         2 => Some(Registers::r2),
//         3 => Some(Registers::r3),
//         4 => Some(Registers::r4),
//         5 => Some(Registers::r5),
//         6 => Some(Registers::r6),
//         7 => Some(Registers::r7),
//         _ => None,
//     }
// }
