use super::register_bank::{RegisterBank, Flags};
use super::memory_bank::MemoryBank;
use super::alu::Alu;

pub struct Mcu {
    reg_bank: RegisterBank,
    memory_bank: MemoryBank,
}

impl Mcu {
    pub fn new() -> Mcu {
        let reg_bank = RegisterBank::new();
        let memory_bank = MemoryBank::new();
        Mcu {reg_bank, memory_bank}
    }

    pub fn step(&mut self) {
        self.execute_step();
        self.reg_bank.increment_pc();
    }

    pub fn get_program_counter(&self) -> u16 {
        self.reg_bank.program_counter
    }

    pub fn load_memory(&mut self, memory: &Vec<u8>) {
        self.memory_bank.set_memory_data(memory)
    }

    pub fn get_register(&self, reg_num: u8) -> u8 {
        self.reg_bank.registers[reg_num as usize]
    }

    pub fn set_register(&mut self, reg_num: u8, value: u8) {
        self.reg_bank.registers[reg_num as usize] = value;
    }

    pub fn get_flags(&self) -> Flags {
        self.reg_bank.get_flags()
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.reg_bank.set_flags(flags);
    }

    fn execute_step(&mut self) {
        let instruction = self.memory_bank.get_word(self.reg_bank.program_counter);
        let decoded = Alu::decode(instruction);
        Alu::execute(&decoded, &mut self.reg_bank, &mut self.memory_bank);
    }
}
