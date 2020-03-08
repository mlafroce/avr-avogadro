use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;

impl Alu {
    pub fn load_immediate(rdu: usize, constant: u8,
        register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = constant;
    }

    pub fn in_out(is_in: bool, reg: u8, address: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        let real_address = address as u16 + 0x20;
        if is_in {
            let data = memory_bank.get_byte(real_address);
            register_bank.registers[reg as usize] = data;
        } else {
            let data = register_bank.registers[reg as usize];
            memory_bank.set_byte(real_address, data);
        }
    }

    pub fn push_pop(is_pop: bool, reg: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        if is_pop {
            register_bank.stack_pointer += 1;
            let data = memory_bank.get_byte(register_bank.stack_pointer);
            register_bank.registers[reg as usize] = data;
        } else {
            if register_bank.stack_pointer == 0 {
                register_bank.stack_pointer = memory_bank.data_size() as u16
            }
            register_bank.stack_pointer -= 1;
            let data = register_bank.registers[reg as usize];
            memory_bank.set_byte(register_bank.stack_pointer, data);
        }
    }

    pub fn transfer_indirect(is_load: bool, is_base_z: bool, reg: u8, offset: u8,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        let base_reg = if is_base_z { 30 } else {28};
        let base_address_lo = register_bank.registers[base_reg as usize];
        let base_address_hi = register_bank.registers[base_reg as usize + 1];
        let address : u16 = ((base_address_hi as u16) << 8) + base_address_lo as u16 + offset as u16;
        if is_load {
            let data = memory_bank.get_byte(address);
            register_bank.registers[reg as usize] = data;
        } else {
            let data = register_bank.registers[reg as usize];
            memory_bank.set_byte(address, data);
        } 
    }
}