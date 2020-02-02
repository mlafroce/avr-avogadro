use super::Instruction;
use super::RawInstruction;
use std::fmt;

/// # Decoder
///
/// Decodes and executes instructions
pub struct Decoder;

const MAIN_OPCODE_MASK: RawInstruction = 0xF000;

impl Decoder {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & MAIN_OPCODE_MASK;  // 4 most sig. bits
        match opcode {
            0x0000 | 0x1000 | 0x2000 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::TwoRegOp{op: raw_instruction >> 10, rd, rr}
                },
            0x4000 | 0x5000 | 0x6000 | 0x7000 | 0xE000 => {
                let rd = ((raw_instruction & 0x00F0) >> 4) as u8;
                let constant_upper = ((raw_instruction & 0x0F00) >> 4) as u8;
                let constant_lower = (raw_instruction & 0x000F) as u8;
                let constant = constant_upper + constant_lower;
                Instruction::RegConstOp{op: raw_instruction >> 12, rd, constant}
            },
            0x9000 => { // One register operations
                match raw_instruction & 0x0E00 {
                    0 | 0x0200 => {
                        let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                        let opcode = (raw_instruction & 0x000F) as u8;
                        let is_load = raw_instruction & 0x0200 == 0;
                        Instruction::Transfer{is_load, reg, opcode}
                    }
                    _ => Instruction::OneRegOp
                }
            },
            0xC000 | 0xD000 => {
                let is_call = opcode == 0xD000;
                let offset = raw_instruction & 0xFFF;
                Instruction::CallJmp{is_call, relative: true, address: offset}
            },
            _ => {
                warn!("Decoding - Unknown opcode: {:x} in {:x}", opcode, raw_instruction);
                Instruction::Unsupported {instruction: raw_instruction}
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::TwoRegOp{op, rd, rr} => 
                write!(f, "Two register operation -> op: {} rd: {}, rr:{}",
                   *op, *rd, *rr),
            Instruction::RegConstOp{op, rd, constant} => 
                write!(f, "Operation against constant -> op: {} rd: {}, constant:{}",
                   *op, *rd, *constant),
            Instruction::Transfer{is_load, reg, opcode} => 
                write!(f, "Transfer operation -> load?: {} reg: {}, subopcode:{}",
                   *is_load, *reg, *opcode),
            Instruction::CallJmp{is_call, relative, address} =>
                write!(f, "Call/jmp operation -> call?: {} relative?: {}, address:{}",
                   *is_call, *relative, *address),
            Instruction::Unsupported {instruction} => 
                write!(f, "Unsupported instruction: {:x}", *instruction),
            _ => write!(f, "Parsed but unsupported instruction")
        }
    }
}
