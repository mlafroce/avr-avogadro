use std::io;
use std::io::{Error, ErrorKind};

pub struct MemoryBank {
    data: Vec<u8>,
    address_mask: u16
}

type AvogadroError = u8;
type Result<T> = std::result::Result<T, AvogadroError>;

impl MemoryBank {
    pub fn new(capacity: u16) -> Result<MemoryBank> {
        if capacity & (capacity - 1) != 0 {
            return Err(1);
        }
        let data = vec![0; capacity as usize];
        let address_mask = capacity - 1;
        Ok(MemoryBank {data, address_mask})
    }

    pub fn set_memory_data(&mut self, data: &Vec<u8>) {
        self.data = data.clone();
    }

    pub fn get_word(&self, address: u16) -> u16 {
        let wrapped_address = address & self.address_mask;
        let mut instruction = self.data[wrapped_address as usize] as u16;
        instruction += (self.data[wrapped_address as usize + 1] as u16) << 8;
        instruction
    }

    pub fn copy_memory(&mut self, data: &Vec<u8>) -> io::Result<()> {
        if self.data.len() >= data.len() {
            self.data[..data.len()].copy_from_slice(&data);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Memory"))
        }
    }
}