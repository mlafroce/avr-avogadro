use super::register_bank::RegisterBank;
use super::memory_bank::MemoryBank;
/// # ALU
///
/// Decodes and executes instructions
pub struct Alu;
/// Arithmetic instructions (sum, substract, etc) and logic (and, or, etc)
mod arithmetic_logic;
/// Transfer instructions (load, store and their variants)
mod transfer;

type RawInstruction = u16;

const RAW_OPCODE_MASK: RawInstruction = 0xF000;

#[derive(Debug)]
pub enum Instruction {
    TwoRegOp {op: RawInstruction, rd: u8, rr: u8},
    RegConstOp {op: RawInstruction, rd: u8, constant: u8},
    Transfer {is_load: bool, reg: u8, opcode: u8},
    OneRegOp,
//    Branch,
//    Bitwise,
    Nop
}

impl Alu {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & RAW_OPCODE_MASK;  // 4 most sig. bits
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
            _ => Instruction::Nop
        }
    }

    /// Executes decoded operation, using registers in register_bank and data
    /// in memory_bank
    pub fn execute(instruction: &Instruction,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        match instruction {
            Instruction::Nop => (),
            Instruction::TwoRegOp{op, rd, rr} => Alu::execute_arithmetic(
                *op, *rd, *rr, register_bank, memory_bank),
            Instruction::RegConstOp{op, rd, constant} => 
                Alu::execute_arith_with_constant(
                *op, *rd, *constant, register_bank),
            Instruction::Transfer{is_load, reg, opcode} =>
                Alu::execute_transfer(*is_load, *reg, *opcode,
                register_bank, memory_bank),
            //Instruction::OneRegOp 
            _ => unimplemented!()
        }
    }

    /// Executes arithmetic instructions
    pub fn execute_arithmetic(op: RawInstruction, rd: u8, rr: u8,
        register_bank: &mut RegisterBank, memory_bank: &MemoryBank) {
        let rdu = rd as usize;
        let rru = rr as usize;
        match op {
            0x1 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::compare(rdu, rru, register_bank, carry)
            },
            0x2 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::substract(rdu, rru, register_bank, carry);
            },
            0x3 => Alu::add(rdu, rru, register_bank, 0),
            0x4 => Alu::comp_skip(rdu, rru, register_bank, memory_bank),
            0x5 => Alu::compare(rdu, rru, register_bank, 0),
            0x6 => Alu::substract(rdu, rru, register_bank, 0),
            0x7 => {
                let carry = register_bank.get_carry_as_u8();
                Alu::add(rdu, rru, register_bank, carry)
            },
            0x8 => Alu::and(rdu, rru, register_bank),
            0x9 => Alu::eor(rdu, rru, register_bank),
            0xA => Alu::or(rdu, rru, register_bank),
            0xB => Alu::mov(rdu, rru, register_bank), 
            _ => unreachable!()
        }
    }

    pub fn execute_arith_with_constant(op: RawInstruction, rd: u8, constant: u8,
        register_bank: &mut RegisterBank) {
        let rdu = rd as usize;
        match op {
            0x4 => {
                Alu::sbci(rdu + 16, constant, register_bank)
            },
            0x5 => {
                Alu::subi(rdu + 16, constant, register_bank)
            },
            0x6 => {
                Alu::ori(rdu + 16, constant, register_bank)
            },
            0x7 => {
                Alu::andi(rdu + 16, constant, register_bank)
            },
            0xE => { // Technically a transfer instruction
                Alu::load_immediate(rdu + 16, constant, register_bank)
            },
            0x96 => {
                Alu::adiw(rdu, constant, register_bank)
            },
            _ => unimplemented!()
        }
    }

    fn execute_transfer(is_load: bool, reg: u8, opcode: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        match opcode {
            0xF => {
                Alu::push_pop(is_load, reg, register_bank, memory_bank);
            },
            _ => unimplemented!()
        }
    }
}
