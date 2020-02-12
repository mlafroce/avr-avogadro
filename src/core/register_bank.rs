///# RegisterBank
///
/// MCU's general purpouse and specific registers
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub neg: bool,
    pub over: bool,
    pub sign: bool,
    pub half: bool,
    pub trans: bool,
    pub int: bool
}

pub struct RegisterBank {
    pub registers: [u8; 32],
    pub program_counter: u16,
    pub stack_pointer: u16,
    flags: Flags
}

const INSTRUCTION_SIZE: u16 = 2;

impl RegisterBank {
    /// Initializes Register bank with zeros, program counter and flags
    pub fn new() -> RegisterBank {
        let registers = [0; 32];
        let program_counter = 0;
        let flags = Flags{carry: false, zero: false, neg: false, over: false,
                         sign: false, half: false, trans: false, int: false};
        let stack_pointer = 0;
        RegisterBank {registers, program_counter, stack_pointer, flags}
    }

    /// Increments program counter by 2, which is the size of an instruction.
    pub fn increment_pc(&mut self) {
        self.program_counter += INSTRUCTION_SIZE;
    }

    /// Program counter getter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Program counter setter
    pub fn set_program_counter(&mut self, pc: u16) {
        self.program_counter = pc;
    }

    /// Stack pointer getter
    pub fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    /// Stack pointer setter
    pub fn set_stack_pointer(&mut self, sp: u16) {
        self.stack_pointer = sp;
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

impl From<Flags> for u8 {
    fn from(flags: Flags) -> u8 {
        let mut result = 0;
        if flags.carry { result += 1 };
        if flags.zero { result += 1 << 1 };
        if flags.neg { result += 1 << 2 };
        if flags.over { result += 1 << 3 };
        if flags.sign { result += 1 << 4 };
        if flags.half { result += 1 << 5 };
        if flags.trans { result += 1 << 6 };
        if flags.int { result += 1 << 7 };
        result
    }
}