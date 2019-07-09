pub struct MemoryBank {
    data: Vec<u8>
}

impl MemoryBank {
    pub fn new() -> MemoryBank {
        let data = Vec::new();
        MemoryBank {data}
    }

    pub fn set_memory_data(&mut self, data: &Vec<u8>) {
        self.data = data.clone();
    }

    pub fn get_instruction(&self, address: u16) -> u16 {
        let mut instruction = self.data[address as usize] as u16;
        instruction += (self.data[address as usize + 1] as u16) << 8;
        instruction
    }
}