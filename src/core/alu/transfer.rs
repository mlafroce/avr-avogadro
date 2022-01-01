use super::Alu;
use crate::core::memory_bank::MemoryBank;
use crate::core::register_bank::RegisterBank;
use crate::core::PointerRegister;

impl Alu {
    pub fn load_immediate(rdu: usize, constant: u8, register_bank: &mut RegisterBank) {
        register_bank.registers[rdu] = constant;
    }

    pub fn in_out(
        is_in: bool,
        reg: u8,
        address: u8,
        register_bank: &mut RegisterBank,
        memory_bank: &mut MemoryBank,
    ) {
        let real_address = address as u16 + 0x20;
        if is_in {
            let data = memory_bank.get_data_byte(real_address);
            register_bank.registers[reg as usize] = data;
        } else {
            let data = register_bank.registers[reg as usize];
            memory_bank.set_data_byte(real_address, data);
        }
    }

    pub fn push_pop(
        is_pop: bool,
        reg: u8,
        register_bank: &mut RegisterBank,
        memory_bank: &mut MemoryBank,
    ) {
        if is_pop {
            register_bank.stack_pointer += 1;
            let data = memory_bank.get_data_byte(register_bank.stack_pointer);
            register_bank.registers[reg as usize] = data;
        } else {
            if register_bank.stack_pointer == 0 {
                register_bank.stack_pointer = memory_bank.data_size() as u16
            }
            register_bank.stack_pointer -= 1;
            let data = register_bank.registers[reg as usize];
            memory_bank.set_data_byte(register_bank.stack_pointer, data);
        }
    }

    pub fn transfer_indirect(
        is_load: bool,
        pointer: PointerRegister,
        reg: u8,
        offset: u8,
        register_bank: &mut RegisterBank,
        memory_bank: &mut MemoryBank,
    ) {
        let pointer = match pointer {
            PointerRegister::X => 26,
            PointerRegister::Y => 28,
            PointerRegister::Z => 30,
        };
        let base_address_lo = register_bank.registers[pointer as usize];
        let base_address_hi = register_bank.registers[pointer as usize + 1];
        let address: u16 = ((base_address_hi as u16) << 8) + base_address_lo as u16 + offset as u16;
        if is_load {
            let data = memory_bank.get_data_byte(address);
            register_bank.registers[reg as usize] = data;
        } else {
            let data = register_bank.registers[reg as usize];
            memory_bank.set_data_byte(address, data);
        }
    }

    pub fn transfer_change_pointer(
        is_load: bool,
        pointer: PointerRegister,
        reg: u8,
        post_inc: bool,
        register_bank: &mut RegisterBank,
        memory_bank: &mut MemoryBank,
    ) {
        if post_inc {
            Alu::transfer_indirect(is_load, pointer, reg, 0, register_bank, memory_bank);
            Alu::increment_pointer(pointer, register_bank);
        } else {
            Alu::decrement_pointer(pointer, register_bank);
            Alu::transfer_indirect(is_load, pointer, reg, 0, register_bank, memory_bank);
        }
    }

    fn decrement_pointer(pointer: PointerRegister, register_bank: &mut RegisterBank) {
        let rd = match pointer {
            PointerRegister::X => 26,
            PointerRegister::Y => 28,
            PointerRegister::Z => 30,
        };
        let rdl = register_bank.registers[rd];
        let rdh = register_bank.registers[rd + 1];
        let resl = rdl.wrapping_sub(1);
        register_bank.registers[rd] = resl;
        if rdl == 0 {
            register_bank.registers[rd + 1] = rdh.wrapping_sub(1);
        }
    }

    fn increment_pointer(pointer: PointerRegister, register_bank: &mut RegisterBank) {
        let rd = match pointer {
            PointerRegister::X => 26,
            PointerRegister::Y => 28,
            PointerRegister::Z => 30,
        };
        let rdl = register_bank.registers[rd];
        let rdh = register_bank.registers[rd + 1];
        let resl = rdl.wrapping_add(1);
        register_bank.registers[rd] = resl;
        if resl == 0 {
            register_bank.registers[rd + 1] = rdh.wrapping_add(1);
        }
    }
}
