use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;
use super::RawInstruction;

const LDS_STS_MASK: RawInstruction = 0xFC0F;

impl Alu {
    // Two Registers instructions
    pub fn add(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        let sum: u16 = register_bank.registers[rdu] as u16 +
            register_bank.registers[rru] as u16 + carry as u16;
        register_bank.registers[rdu] = sum as u8;
        let mut flags = register_bank.get_flags();
        flags.carry = sum > 0xFF;
        flags.zero = (sum & 0xFF) == 0;
        register_bank.set_flags(flags);
    }

    pub fn compare(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        Alu::_substract_base(rdu, rru, register_bank, carry, false);
    }

    pub fn comp_skip(rdu: usize, rru: usize, register_bank: &mut RegisterBank,
        memory_bank: &MemoryBank) {
        let rd_value = register_bank.registers[rdu];
        let rr_value = register_bank.registers[rru];
        if rd_value == rr_value {
            register_bank.increment_pc();
        }
        // If next instruction is `LDS` or `STS`, should skip again
        let pc = register_bank.get_program_counter();
        let next_instruction = memory_bank.get_word(pc);
        if next_instruction & LDS_STS_MASK == 0x9000 {
            register_bank.increment_pc();
        }
    }

    pub fn substract(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        Alu::_substract_base(rdu, rru, register_bank, carry, true);
    }

    pub fn _substract_base(rdu: usize, rru: usize, register_bank: &mut RegisterBank,
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

    pub fn and(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let rr_value = register_bank.registers[rru];
        Alu::andi(rdu, rr_value, register_bank);
    }

    pub fn andi(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] & constant;
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    pub fn eor(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] ^
            register_bank.registers[rru];
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    pub fn or(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let rr_value = register_bank.registers[rru];
        Alu::ori(rdu, rr_value, register_bank);
    }

    pub fn ori(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] | constant;
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        register_bank.set_flags(flags);
    }

    pub fn mov(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = register_bank.registers[rru];
    }

    // One register - One constant operations

    /// Substracts immediate to register
    pub fn subi(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        Alu::_substract_imm_base(rdu, constant, register_bank, 0);
    }

    /// Substracts immediate to register with carry
    pub fn sbci(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let carry = register_bank.get_carry_as_u8();
        Alu::_substract_imm_base(rdu, constant, register_bank, carry);
    }

    pub fn _substract_imm_base(rdu: usize, constant: u8,
        register_bank: &mut RegisterBank, carry: u8) {
        let const_with_carry = constant.wrapping_add(carry) as u16;
        let result: u16 = (register_bank.registers[rdu] as u16)
            .wrapping_sub(const_with_carry);
        register_bank.registers[rdu] = result as u8;
        let mut flags = register_bank.get_flags();
        flags.zero = register_bank.registers[rdu] == 0;
        register_bank.set_flags(flags);
    }

    /// Adds immediate to word
    /// Available on families >= AVR2
    pub fn adiw(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let rd = 24 + rdu * 2;
        let rdl = register_bank.registers[rd] as u16;
        let rdh = register_bank.registers[rd+1];
        let sum: u16 = rdl + constant as u16;
        register_bank.registers[rdu] = sum as u8;
        if sum > 0xFF {
            register_bank.registers[rd+1] = rdh + 1
        }
        let mut flags = register_bank.get_flags();
        flags.zero = register_bank.registers[rd+1] == 0 &&
                      register_bank.registers[rd] == 0;
        register_bank.set_flags(flags);
        // TODO: TEST!
        unimplemented!();
    }
}