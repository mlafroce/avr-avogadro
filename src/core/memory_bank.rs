use std::io;
use std::io::{Error, ErrorKind};

/// Microcontroller main memory
pub struct MemoryBank {
    data: Vec<u8>,
    address_mask: u16
}

type AvogadroError = u8;
type Result<T> = std::result::Result<T, AvogadroError>;

impl MemoryBank {
    /// Creates a new memory bank. Capacity *MUST* be a power of 2
    pub fn new(capacity: u16) -> Result<MemoryBank> {
        if capacity & (capacity - 1) != 0 {
            return Err(1);
        }
        let data = vec![0; capacity as usize];
        let address_mask = capacity - 1;
        Ok(MemoryBank {data, address_mask})
    }

    pub fn set_memory_data(&mut self, data: &[u8]) {
        self.data = data.to_owned();
    }

    /// Returns a byte located at `address` position
    pub fn get_byte(&self, address: u16) -> u8 {
        let wrapped_address = address & self.address_mask;
        self.data[wrapped_address as usize]
    }

    /// Sets a byte at `address` position 
    pub fn set_byte(&mut self, address: u16, data: u8) {
        let wrapped_address = address & self.address_mask;
        self.data[wrapped_address as usize] = data
    }

    /// Returns a 2 byte word located at `address` 
    pub fn get_word(&self, address: u16) -> u16 {
        let wrapped_address = address & self.address_mask;
        let instruction = u16::from(self.data[wrapped_address as usize]);
        instruction + ((u16::from(self.data[wrapped_address as usize + 1])) << 8)
    }

    /// Copies values at array `data` into memory bank. If memory bank
    /// is smaller, throws error.
    pub fn copy_memory(&mut self, data: &[u8]) -> io::Result<()> {
        if self.data.len() >= data.len() {
            self.data[..data.len()].copy_from_slice(&data);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Memory"))
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}