use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;

impl Alu {
    pub fn ret(_is_interruption: bool, register_bank: &mut RegisterBank,
        memory_bank: &MemoryBank) {
        register_bank.stack_pointer += 2  as u16;
        if register_bank.stack_pointer >= (memory_bank.data_size() - 1) as u16 {
            register_bank.stack_pointer = 0;
        }
        let pc_lo = memory_bank.get_data_byte(register_bank.stack_pointer);
        let pc_hi = memory_bank.get_data_byte(register_bank.stack_pointer + 1) as u16;
        register_bank.set_program_counter((pc_hi << 8) + pc_lo as u16);
    }
}
