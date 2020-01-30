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
    pub sp: u16,
    flags: Flags
}

const INSTRUCTION_SIZE: u16 = 2;

impl RegisterBank {
    /// Initializes Register bank with zeros, program counter and flags
    pub fn new() -> RegisterBank {
        let registers = [0; 32];
        let program_counter = 0;
        let flags = Flags{carry: false, zero: false};
        let sp = 0;
        RegisterBank {registers, program_counter, sp, flags}
    }

    /// Increments program counter by 2, which is the size of an instruction.
    pub fn increment_pc(&mut self) {
        self.program_counter += INSTRUCTION_SIZE;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, pc: u16) {
        self.program_counter = pc;
    }

    /// Flags getter
    pub fn get_flags(&self) -> Flags {
        self.flags
    }

    /// Flags setter
    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    /// Returns 1 if carry flag is true, otherwise 0
    pub fn get_carry_as_u8(&self) -> u8 {
        if self.get_flags().carry {1} else {0}
    }
}

impl Default for RegisterBank {
    fn default() -> Self {
        Self::new()
    }
}
