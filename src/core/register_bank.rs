///# RegisterBank
///
/// MCU's general purpouse and specific registers
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
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
        let flags = Flags{carry: false, zero: false};
        RegisterBank {registers, program_counter, flags}
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += INSTRUCTION_SIZE;
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    pub fn get_flags(&self) -> Flags {
        self.flags
    }

    pub fn get_carry_as_u8(&self) -> u8 {
        if self.get_flags().carry {1} else {0}
    }
}