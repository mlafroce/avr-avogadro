use crate::core::register_bank::RegisterBank;
use crate::core::memory_bank::MemoryBank;
use super::Alu;
use super::RawInstruction;

const LDS_STS_MASK: RawInstruction = 0xFC0F;

impl Alu {
    pub fn execute_calljmp(is_call: bool, relative: bool, address: u16,
        register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        let pc = register_bank.get_program_counter();
        if relative {
            let address_offset: i16 = if address & 0x800 == 0 {
                address as i16
            } else {
                address as i16 - 0x1000
            };
            let new_pc = pc as i16 + address_offset;
            if new_pc < 0 {
                register_bank.set_program_counter((memory_bank.size() as i16 + new_pc) as u16);
            } else {
                register_bank.set_program_counter(new_pc as u16);
            }
        } else {
            warn!("call jmp absolute not implemented");
        }
        if is_call {
            if register_bank.stack_pointer < 2 {
                register_bank.stack_pointer = memory_bank.size() as u16
            }
            register_bank.stack_pointer = register_bank.stack_pointer - 2;
            memory_bank.set_byte(register_bank.stack_pointer, pc.to_le_bytes()[0]);
            memory_bank.set_byte(register_bank.stack_pointer + 1, pc.to_le_bytes()[1]);
        }
    }
}