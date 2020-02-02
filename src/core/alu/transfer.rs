use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;

impl Alu {
    pub fn load_immediate(rdu: usize, constant: u8,
        register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = constant;
    }

    pub fn push_pop(is_load: bool, reg: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        if is_load {
            register_bank.stack_pointer += 1;
            let data = memory_bank.get_byte(register_bank.stack_pointer);
            register_bank.registers[reg as usize] = data;
        } else {
            if register_bank.stack_pointer == 0 {
                register_bank.stack_pointer = memory_bank.size() as u16
            }
            register_bank.stack_pointer -= 1;
            let data = register_bank.registers[reg as usize];
            memory_bank.set_byte(register_bank.stack_pointer, data);
        }
    }
}