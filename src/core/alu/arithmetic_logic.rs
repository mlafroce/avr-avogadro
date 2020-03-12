use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;
use super::RawInstruction;

const LDS_STS_MASK: RawInstruction = 0xFC0F;

impl Alu {
    // Two Registers instructions
    pub fn add(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        let rd = register_bank.registers[rdu];
        let rr = register_bank.registers[rru];
        let sum = rd.wrapping_add(rr).wrapping_add(carry as u8);
        let hc_flags = (rd & rr) | (rr & !sum) | (rd & !sum);
        register_bank.registers[rdu] = sum;
        let mut flags = register_bank.get_flags();
        flags.carry = hc_flags & 0x80 != 0;
        flags.half = hc_flags & 0x08 != 0;
        flags.neg = sum & 0x80 != 0;
        let tmp_overflow = (rd & rr & !sum) | (!rd & !rr & sum);
        flags.over = tmp_overflow & 0x80 != 0; 
        flags.zero = sum == 0;
        flags.sign = flags.neg ^ flags.over;
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
        let next_instruction = memory_bank.get_program_word(pc);
        if next_instruction & LDS_STS_MASK == 0x9000 {
            register_bank.increment_pc();
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
        flags.neg = result & 0x80 != 0;
        flags.over = false;
        flags.sign = flags.neg;
        register_bank.set_flags(flags);
    }

    pub fn eor(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        let result = register_bank.registers[rdu] ^
            register_bank.registers[rru];
        register_bank.registers[rdu] = result;
        let mut flags = register_bank.get_flags();
        flags.zero = result == 0;
        flags.neg = result & 0x80 != 0;
        flags.over = false;
        flags.sign = flags.neg;
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
        flags.neg = result & 0x80 != 0;
        flags.over = false;
        flags.sign = flags.neg;
        register_bank.set_flags(flags);
    }

    pub fn mov(rdu: usize, rru: usize, register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = register_bank.registers[rru];
    }

    // One register - One constant operations
    /// Substracts immediate to register
    pub fn subi(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        Alu::_substract_imm_base(rdu, constant, register_bank, 0, true);
    }

    /// Substracts immediate to register with carry
    pub fn sbci(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let carry = register_bank.get_carry_as_u8();
        Alu::_substract_imm_base(rdu, constant, register_bank, carry, true);
    }

    /// Substracts immediate to register with carry
    pub fn cpi(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let carry = register_bank.get_carry_as_u8();
        Alu::_substract_imm_base(rdu, constant, register_bank, carry, false);
    }

    pub fn substract(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        Alu::_substract_base(rdu, rru, register_bank, carry, true);
    }

    pub fn _substract_base(rdu: usize, rru: usize, register_bank: &mut RegisterBank,
        carry: u8, store_result: bool) {
        // wrapping sub as it could overflow
        let rd = register_bank.registers[rdu] as u8;
        let rr = register_bank.registers[rru];
        let rr_plus_c = rr.wrapping_add(carry as u8);
        let result = rd.wrapping_sub(rr_plus_c);
        let hc_flags = (!rd & rr) | (rr & result) | (!rd & result);
        let mut flags = register_bank.get_flags();
        flags.carry = hc_flags & 0x80 != 0;
        flags.half = hc_flags & 0x08 != 0;
        flags.neg = result & 0x80 != 0;
        let tmp_overflow = (rd & !rr & !result) | (!rd & rr & result);
        flags.over = tmp_overflow & 0x80 != 0; 
        flags.zero = result == 0;
        flags.sign = flags.neg ^ flags.over;
        register_bank.set_flags(flags);
        if store_result {
            register_bank.registers[rdu] = result;
        }
    }

    pub fn _substract_imm_base(rdu: usize, constant: u8,
        register_bank: &mut RegisterBank, carry: u8, store_result: bool) {
        let rd = register_bank.registers[rdu] as u8;
        let const_with_carry = constant.wrapping_add(carry);
        let result = rd.wrapping_sub(const_with_carry);
        let hc_flags = (!rd & const_with_carry) | (const_with_carry & result) | (!rd & result);
        let mut flags = register_bank.get_flags();
        flags.carry = hc_flags & 0x80 != 0;
        flags.half = hc_flags & 0x08 != 0;
        flags.neg = result & 0x80 != 0;
        let tmp_overflow = (rd & !const_with_carry & !result) | (!rd & const_with_carry & result);
        flags.over = tmp_overflow & 0x80 != 0; 
        flags.zero = result == 0;
        flags.sign = flags.neg ^ flags.over;
        register_bank.set_flags(flags);
        if store_result {
            register_bank.registers[rdu] = result;
        }
    }

    /// Adds immediate to word
    /// Available on families >= AVR2
    pub fn adiw(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let rd = 24 + rdu * 2;
        let rdl = register_bank.registers[rd];
        let rdh = register_bank.registers[rd+1];
        let resl = rdl.wrapping_add(constant);
        register_bank.registers[rd] = resl;
        if resl < rdl && resl < constant {
            register_bank.registers[rd+1] = rdh.wrapping_add(1);
        }
        let resh = register_bank.registers[rd+1];
        let mut flags = register_bank.get_flags();
        flags.carry = (rdh & !resh) & 0x80 != 0;
        flags.over = (!rdh & resh) & 0x80 != 0;
        flags.neg = resh & 0x80 != 0;
        flags.zero = resh == 0 && resl == 0;
        flags.sign = flags.neg ^ flags.over;
        register_bank.set_flags(flags);
    }

    /// Substract immediate to word
    /// Available on families >= AVR2
    pub fn sbiw(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        let rd = 24 + rdu * 2;
        let rdl = register_bank.registers[rd];
        let rdh = register_bank.registers[rd+1];
        let resl = rdl.wrapping_sub(constant);
        register_bank.registers[rd] = resl;
        if constant > rdl {
            register_bank.registers[rd+1] = rdh.wrapping_sub(1);
        }
        let resh = register_bank.registers[rd+1];
        println!("rd({}), rd+1: {:x}{:x}", rd, resh, resl);
        let mut flags = register_bank.get_flags();
        flags.carry = (!rdh & resh) & 0x80 != 0;
        flags.over = (rdh & !resh) & 0x80 != 0;
        flags.neg = resh & 0x80 != 0;
        flags.zero = resh == 0 && resl == 0;
        flags.sign = flags.neg ^ flags.over;
        register_bank.set_flags(flags);
    }
}