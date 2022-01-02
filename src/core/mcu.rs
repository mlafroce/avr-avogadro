use super::alu::Alu;
use super::decoder::Decoder;
use super::memory_bank::MemoryBank;
use super::register_bank::{Flags, RegisterBank};

use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::Read;
use std::slice::from_raw_parts_mut;

pub struct Mcu {
    memory_bank: MemoryBank,
    reg_bank: RegisterBank,
    cycle_count: usize,
}

impl Mcu {
    pub fn new(data_size: usize, program_size: usize) -> Mcu {
        let memory_bank = MemoryBank::new(data_size, program_size).unwrap();
        let reg_bank = RegisterBank::new();
        let cycle_count = 0;
        Mcu {
            reg_bank,
            memory_bank,
            cycle_count,
        }
    }

    pub fn step(&mut self) {
        self.execute_step();
        self.reg_bank.increment_pc(&self.memory_bank);
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.execute_step();
            self.reg_bank.increment_pc(&self.memory_bank);
        }
    }

    pub fn load_data_memory(&mut self, memory: &[u8]) {
        self.memory_bank.copy_into_data_memory(memory);
    }

    pub fn load_program_memory(&mut self, memory: &[u8]) {
        self.memory_bank.copy_into_program_memory(memory);
    }

    pub fn load_ihex_file(&mut self, filename: &str) -> io::Result<()> {
        let mut buffer = String::new();
        let mut file = File::open(filename)?;
        file.read_to_string(&mut buffer)?;
        let reader = ihex::Reader::new(&buffer);
        let buffer = reader
            .flatten()
            .filter_map(|record| {
                if let ihex::Record::Data { offset: _, value } = record {
                    Some(value)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        self.memory_bank.copy_into_program_memory(&buffer);
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str, is_program: bool) -> io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if is_program {
            self.memory_bank.copy_into_program_memory(&buffer);
        } else {
            self.memory_bank.copy_into_data_memory(&buffer);
        }
        Ok(())
    }

    /// Returns size of main memory bank, in bytes
    pub fn get_data_size(&self) -> usize {
        self.memory_bank.data_size()
    }

    /// Returns size of main memory bank, in bytes
    pub fn get_program_size(&self) -> usize {
        self.memory_bank.program_size()
    }

    pub fn get_data_byte(&self, address: u16) -> u8 {
        self.memory_bank.get_data_byte(address)
    }

    /// Copies content from data memory into buffer array.
    /// If buffer is smaller than memory copies at most *buf_size* elements.
    /// # Safety
    ///
    /// `buffer` must be an array with size in bytes equals or larger than buf_size
    pub unsafe fn get_data_memory(&self, buffer: *mut u8, buf_size: usize) {
        let slice = from_raw_parts_mut(buffer, buf_size);
        self.memory_bank.copy_from_data_memory(slice);
    }

    /// Copies content from data memory into buffer array.
    /// If buffer is smaller than memory copies at most *buf_size* elements.
    /// # Safety
    ///
    /// `buffer` must be an array with size in bytes equals or larger than buf_size
    pub unsafe fn get_program_memory(&self, buffer: *mut u8, buf_size: usize) {
        let slice = from_raw_parts_mut(buffer, buf_size);
        self.memory_bank.copy_from_program_memory(slice);
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
        self.reg_bank.get_program_counter()
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.reg_bank.set_program_counter(value);
    }

    pub fn get_current_instruction(&self) -> u16 {
        self.fetch()
    }

    pub fn get_stack_pointer(&self) -> u16 {
        self.reg_bank.get_stack_pointer()
    }

    pub fn display_current_instruction(&self, buf: &mut String) {
        let instruction = self.fetch();
        let decoded = Decoder::decode(instruction);
        write!(buf, "{}", decoded).unwrap();
    }

    pub fn get_flags(&self) -> Flags {
        self.reg_bank.get_flags()
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.reg_bank.set_flags(flags);
    }

    fn execute_step(&mut self) {
        let instruction = self.fetch();
        let decoded = Decoder::decode(instruction);
        Alu::execute(&decoded, &mut self.reg_bank, &mut self.memory_bank);
    }

    fn fetch(&self) -> u16 {
        self.memory_bank
            .get_program_word(self.reg_bank.get_program_counter())
    }
}
