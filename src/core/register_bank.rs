///# RegisterBank
///
/// MCU's general purpouse and specific registers

struct Flags {
    carry: bool,
}

pub struct RegisterBank {
    pub registers: [u8; 32],
    pub program_counter: u16,
    flags: Flags
}

const INSTRUCTION_SIZE: u16 = 2;

impl RegisterBank {
    pub fn new() -> RegisterBank {
        let registers = [0; 32];
        let program_counter = 0;
        let flags = Flags{carry: false};
        RegisterBank {registers, program_counter, flags}
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += INSTRUCTION_SIZE;
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.flags.carry = carry;
    }

    pub fn get_carry(&self) -> bool {
        self.flags.carry
    }
}