#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// 16-bit virtual machine: the opcode lives in the top 4 bits.
pub fn get_instruction(instruction: u16) -> InstructionSet {
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

fn opcode(mnemonic: &str) -> Option<u16> {
    let code = match mnemonic {
        "NOP" => 0,
        "MOV" => 1,
        "MOVR" => 2,
        "ADD" => 3,
        "SUB" => 4,
        "MUL" => 5,
        "DIV" => 6,
        "AND" => 7,
        "OR" => 8,
        "XOR" => 9,
        "NOT" => 10,
        "JMP" => 11,
        "JEQ" => 12,
        "JNE" => 13,
        "LOAD" => 14,
        "STORE" => 15,
        _ => return None,
    };
    Some(code)
}

fn parse_register(token: &str) -> Result<u16, String> {
    let idx = token
        .strip_prefix('r')
        .or_else(|| token.strip_prefix('R'))
        .ok_or_else(|| format!("expected a register like r0, got `{token}`"))?
        .parse::<u16>()
        .map_err(|_| format!("invalid register `{token}`"))?;
    if idx > 7 {
        return Err(format!("register out of range (r0-r7): `{token}`"));
    }
    Ok(idx)
}

fn parse_immediate(token: &str) -> Result<u16, String> {
    let value = if let Some(hex) = token.strip_prefix("0x").or_else(|| token.strip_prefix("0X")) {
        u16::from_str_radix(hex, 16)
    } else {
        token.parse::<u16>()
    };
    value.map_err(|_| format!("invalid immediate `{token}`"))
}

/// Assemble a single line of assembly into a machine instruction.
///
/// Blank lines and `;` comments produce `Ok(None)`.
pub fn assemble_line(line: &str) -> Result<Option<u16>, String> {
    let line = line.split(';').next().unwrap_or("").trim();
    if line.is_empty() {
        return Ok(None);
    }

    let cleaned = line.replace(',', " ");
    let mut tokens = cleaned.split_ascii_whitespace();
    let mnemonic = tokens.next().unwrap().to_ascii_uppercase();
    let op = opcode(&mnemonic).ok_or_else(|| format!("unknown instruction `{mnemonic}`"))?;
    let args: Vec<&str> = tokens.collect();

    let encoded = match mnemonic.as_str() {
        "NOP" => op << 12,
        "MOV" => {
            expect_args(&mnemonic, &args, 2)?;
            let rd = parse_register(args[0])?;
            let imm = parse_immediate(args[1])?;
            if imm > 0xFF {
                return Err(format!("MOV immediate must fit in 8 bits: `{}`", args[1]));
            }
            (op << 12) | (rd << 8) | (imm & 0xFF)
        }
        "MOVR" | "NOT" => {
            expect_args(&mnemonic, &args, 2)?;
            let rd = parse_register(args[0])?;
            let rs = parse_register(args[1])?;
            (op << 12) | (rd << 8) | (rs << 4)
        }
        "ADD" | "SUB" | "MUL" | "DIV" | "AND" | "OR" | "XOR" => {
            expect_args(&mnemonic, &args, 3)?;
            let rd = parse_register(args[0])?;
            let rs = parse_register(args[1])?;
            let rt = parse_register(args[2])?;
            (op << 12) | (rd << 8) | (rs << 4) | rt
        }
        "JMP" => {
            expect_args(&mnemonic, &args, 1)?;
            let addr = parse_immediate(args[0])?;
            if addr > 0xFFF {
                return Err(format!("JMP address must fit in 12 bits: `{}`", args[0]));
            }
            (op << 12) | (addr & 0xFFF)
        }
        "JEQ" | "JNE" => {
            expect_args(&mnemonic, &args, 3)?;
            let rs = parse_register(args[0])?;
            let rt = parse_register(args[1])?;
            let addr = parse_immediate(args[2])?;
            if addr > 0xF {
                return Err(format!("branch address must fit in 4 bits: `{}`", args[2]));
            }
            (op << 12) | (rs << 8) | (rt << 4) | (addr & 0xF)
        }
        "LOAD" | "STORE" => {
            expect_args(&mnemonic, &args, 2)?;
            let reg = parse_register(args[0])?;
            let addr = parse_immediate(args[1])?;
            if addr > 0xFF {
                return Err(format!("memory address must fit in 8 bits: `{}`", args[1]));
            }
            (op << 12) | (reg << 8) | (addr & 0xFF)
        }
        _ => unreachable!("opcode lookup already validated the mnemonic"),
    };

    Ok(Some(encoded))
}

fn expect_args(mnemonic: &str, args: &[&str], n: usize) -> Result<(), String> {
    if args.len() != n {
        return Err(format!(
            "{mnemonic} expects {n} operand(s), got {}",
            args.len()
        ));
    }
    Ok(())
}

/// Assemble a full program, one instruction per non-empty line.
pub fn assemble(source: &str) -> Result<Vec<u16>, String> {
    let mut program = Vec::new();
    for (i, line) in source.lines().enumerate() {
        match assemble_line(line) {
            Ok(Some(word)) => program.push(word),
            Ok(None) => {}
            Err(e) => return Err(format!("line {}: {e}", i + 1)),
        }
    }
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembles_arithmetic() {
        assert_eq!(assemble_line("MOV r1, 2").unwrap(), Some(0x1102));
        assert_eq!(assemble_line("ADD r0, r1, r2").unwrap(), Some(0x3012));
        assert_eq!(assemble_line("MUL r0, r1, r2").unwrap(), Some(0x5012));
    }

    #[test]
    fn skips_comments_and_blanks() {
        assert_eq!(assemble_line("  ; just a comment").unwrap(), None);
        assert_eq!(assemble_line("").unwrap(), None);
    }

    #[test]
    fn hex_immediates() {
        assert_eq!(assemble_line("MOV r1, 0x0F").unwrap(), Some(0x110F));
    }

    #[test]
    fn rejects_bad_register() {
        assert!(assemble_line("MOV r9, 1").is_err());
    }

    #[test]
    fn assembles_program() {
        let src = "MOV r1, 2\nMOV r2, 3\nADD r0, r1, r2\n";
        assert_eq!(assemble(src).unwrap(), vec![0x1102, 0x1203, 0x3012]);
    }
}
