use super::Alu;
use crate::core::memory_bank::MemoryBank;
use crate::core::register_bank::RegisterBank;

impl Alu {
    pub fn execute_calljmp(
        is_call: bool,
        relative: bool,
        address_bits: u16,
        register_bank: &mut RegisterBank,
        memory_bank: &mut MemoryBank,
    ) {
        let pc = register_bank.get_program_counter();
        if relative {
            let address_offset: i16 = if address_bits & 0x800 == 0 {
                address_bits as i16
            } else {
                address_bits as i16 - 0x1000
            };
            let new_pc = pc as i16 + address_offset * 2;
            if new_pc < 0 {
                register_bank
                    .set_program_counter((memory_bank.program_size() as i16 + new_pc) as u16);
            } else {
                register_bank.set_program_counter(new_pc as u16);
            }
        } else {
            warn!("call jmp absolute not implemented");
        }
        if is_call {
            let pc_to_store = (pc + 2).to_le_bytes();
            memory_bank.set_data_byte(register_bank.stack_pointer, pc_to_store[0]);
            memory_bank.set_data_byte(register_bank.stack_pointer + 1, pc_to_store[1]);
            if register_bank.stack_pointer < 2 {
                register_bank.stack_pointer = memory_bank.data_size() as u16;
            }
            register_bank.stack_pointer -= 2;
        }
    }
}
