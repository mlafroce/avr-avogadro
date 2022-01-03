/// Microcontroller main memory
pub struct MemoryBank {
    data_memory: Vec<u8>,
    program_memory: Vec<u8>,
    address_mask: u16,
}

type AvogadroError = u8;
type Result<T> = std::result::Result<T, AvogadroError>;

impl MemoryBank {
    /// Creates a new memory bank. Capacity *MUST* be a power of 2
    pub fn new(data_size: usize, program_size: usize) -> Result<MemoryBank> {
        if data_size & (data_size - 1) != 0 {
            return Err(1);
        }
        let data_memory = vec![0; data_size];
        let program_memory = vec![0; program_size];
        let address_mask = (data_size - 1) as u16;
        Ok(MemoryBank {
            data_memory,
            program_memory,
            address_mask,
        })
    }

    pub fn set_data_memory(&mut self, data: &[u8]) {
        self.data_memory = data.to_owned();
    }

    pub fn set_program_memory(&mut self, data: &[u8]) {
        self.program_memory = data.to_owned();
    }

    /// Returns a byte located at `address` position
    pub fn get_data_byte(&self, address: u16) -> u8 {
        let wrapped_address = address & self.address_mask;
        self.data_memory[wrapped_address as usize]
    }

    /// Sets a byte at `address` position
    pub fn set_data_byte(&mut self, address: u16, data: u8) {
        let wrapped_address = address & self.address_mask;
        self.data_memory[wrapped_address as usize] = data
    }

    /// Returns a 2 byte word located at `address`
    pub fn get_program_word(&self, address: u16) -> u16 {
        let wrapped_address = address & self.address_mask;
        let instruction = u16::from(self.program_memory[wrapped_address as usize]);
        instruction + ((u16::from(self.program_memory[wrapped_address as usize + 1])) << 8)
    }

    /// Copies values at array `data` into memory bank.
    pub fn copy_into_data_memory(&mut self, data: &[u8]) {
        let n_bytes = std::cmp::min(data.len(), self.data_memory.len());
        self.data_memory[..n_bytes].copy_from_slice(&data);
    }

    /// Copies values from memory bank into array `data`.
    pub fn copy_from_data_memory(&self, data: &mut [u8]) {
        let n_bytes = std::cmp::min(data.len(), self.data_memory.len());
        data[..n_bytes].copy_from_slice(&self.data_memory);
    }

    /// Copies values at array `data` into memory bank.
    pub fn copy_into_program_memory(&mut self, data: &[u8]) {
        let n_bytes = std::cmp::min(data.len(), self.program_memory.len());
        self.program_memory[..n_bytes].copy_from_slice(&data);
    }

    /// Copies values from memory bank into array `data`.
    pub fn copy_from_program_memory(&self, data: &mut [u8]) {
        let n_bytes = std::cmp::min(data.len(), self.data_memory.len());
        data[..n_bytes].copy_from_slice(&self.data_memory);
    }

    /// SRAM memory size in bytes
    pub fn data_size(&self) -> usize {
        self.data_memory.len()
    }

    /// Flash memory size in bytes
    pub fn program_size(&self) -> usize {
        self.program_memory.len()
    }
}
