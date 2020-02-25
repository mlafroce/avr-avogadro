use crate::core::register_bank::RegisterBank;
use super::Alu;


impl Alu {
    // Two Registers instructions
    pub fn execute_branch(op: u8, test_set: bool,
        offset: i8, register_bank: &mut RegisterBank) {
        let flags = register_bank.get_flags();
        let mut pc = register_bank.get_program_counter() as i16;
        match op {
            0x0 => if flags.carry == test_set { pc += offset as i16 * 2},
            0x1 => if flags.zero == test_set { pc += offset as i16 * 2},
            0x2 => if flags.neg  == test_set { pc += offset as i16 * 2},
            0x3 => if flags.over == test_set { pc += offset as i16 * 2},
            0x4 => if flags.sign == test_set { pc += offset as i16 * 2},
            0x5 => if flags.half == test_set { pc += offset as i16 * 2},
            0x6 => if flags.trans == test_set { pc += offset as i16 * 2},
            0x7 => if flags.int  == test_set { pc += offset as i16 * 2},
            _ => unreachable!()
        }
        register_bank.set_program_counter(pc as u16);
    }
}