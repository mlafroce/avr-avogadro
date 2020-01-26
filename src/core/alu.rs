use super::register_bank::RegisterBank;
use super::memory_bank::MemoryBank;
/// # ALU
///
/// Decodes and executes instructions
pub struct Alu;

type RawInstruction = u16;

const RAW_OPCODE_MASK: RawInstruction = 0xF000;
const LDS_STS_MASK: RawInstruction = 0xFC0F;

#[derive(Debug)]
pub enum Instruction {
    TwoOperand {op: RawInstruction, rd: u8, rr: u8},
//    Branch,
//    Transfer,
//    Bitwise,
    Nop
}

impl Alu {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & RAW_OPCODE_MASK;
        match opcode {
            0x0000 | 0x1000 | 0x2000 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::TwoOperand{op: raw_instruction >> 10, rd, rr}
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
            Instruction::TwoOperand{op, rd, rr} => Alu::execute_arithmetic(
                *op, *rd, *rr, register_bank, memory_bank)
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

    fn add(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        let sum: u16 = register_bank.registers[rdu] as u16 +
            register_bank.registers[rru] as u16 + carry as u16;
        register_bank.registers[rdu] = sum as u8;
        let mut flags = register_bank.get_flags();
        flags.carry = sum > 0xFF;
        flags.zero = (sum & 0xFF) == 0;
        register_bank.set_flags(flags);
    }

    fn compare(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        Alu::_substract_base(rdu, rru, register_bank, carry, false);
    }

    fn comp_skip(rdu: usize, rru: usize, register_bank: &mut RegisterBank,
        memory_bank: &MemoryBank) {
        let rd_value = register_bank.registers[rdu];
        let rr_value = register_bank.registers[rru];
        if rd_value == rr_value {
            register_bank.increment_pc();
        }
        // If next instruction is `LDS` or `STS`, should skip again
        let next_instruction = memory_bank.get_word(register_bank.program_counter);
        if next_instruction & LDS_STS_MASK == 0x9000 {
            register_bank.increment_pc();
        }
    }

    fn substract(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        Alu::_substract_base(rdu, rru, register_bank, carry, true);
    }

    fn _substract_base(rdu: usize, rru: usize, register_bank: &mut RegisterBank,
        carry: u8, store_result: bool) {
        // wrapping sub as it could overflow
        let rr_plus_c = (register_bank.registers[rru] as u8)
            .wrapping_add(carry as u8) as u16;
        let result: u16 = (register_bank.registers[rdu] as u16)
            .wrapping_sub(rr_plus_c);
        let mut flags = register_bank.get_flags();
        flags.carry = result > 0xFF;
        flags.zero = (result & 0xFF) == 0;
        register_bank.set_flags(flags);
        if store_result {
            register_bank.registers[rdu] = result as u8;
        }
    }

    fn and(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] &
            register_bank.registers[rru];
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    fn eor(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] ^
            register_bank.registers[rru];
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    fn or(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] |
            register_bank.registers[rru];
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    fn mov(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = register_bank.registers[rru];
    }
}
