use super::register_bank::{RegisterBank, Flags};
use super::memory_bank::MemoryBank;
use super::alu::Alu;

use std::fs::File;
use std::io;
use std::io::Read;

const MEMORY_INITIAL_SIZE: u16 = 1024;

pub struct Mcu {
    reg_bank: RegisterBank,
    memory_bank: MemoryBank,
}

impl Mcu {
    pub fn new() -> Mcu {
        let reg_bank = RegisterBank::new();
        let memory_bank = MemoryBank::new(MEMORY_INITIAL_SIZE).unwrap();
        Mcu {reg_bank, memory_bank}
    }

    pub fn step(&mut self) {
        self.execute_step();
        self.reg_bank.increment_pc();
    }

    pub fn load_memory(&mut self, memory: &[u8]) {
        self.memory_bank.set_memory_data(memory)
    }

    pub fn load_memory_from_file(&mut self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.memory_bank.copy_memory(&buffer)
    }

    pub fn get_register(&self, reg_num: u8) -> u8 {
        self.reg_bank.registers[reg_num as usize]
    }

    pub fn set_register(&mut self, reg_num: u8, value: u8) {
        self.reg_bank.registers[reg_num as usize] = value;
    }

    pub fn get_register_array(&self) -> [u8; 32] {
        self.reg_bank.registers
    }

    pub fn set_register_array(&mut self, reg_array: [u8; 32]) {
        self.reg_bank.registers = reg_array;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.reg_bank.program_counter
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.reg_bank.program_counter = value;
    }

    pub fn get_flags(&self) -> Flags {
        self.reg_bank.get_flags()
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.reg_bank.set_flags(flags);
    }

    fn execute_step(&mut self) {
        let instruction = self.fetch();
        let decoded = Alu::decode(instruction);
        Alu::execute(&decoded, &mut self.reg_bank, &mut self.memory_bank);
    }

    fn fetch(&self) -> u16 {
        self.memory_bank.get_word(self.reg_bank.program_counter)
    }
}

impl Default for Mcu {
    fn default() -> Self {
        Self::new()
    }
}
