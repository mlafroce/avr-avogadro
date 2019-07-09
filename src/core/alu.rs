use super::register_bank::RegisterBank;
use super::memory_bank::MemoryBank;
/// # ALU
///
/// Decodes and executes instructions
pub struct Alu;

#[derive(Debug)]
pub enum Instruction {
    Arithmetic {op: u16, rd: u8, rr: u8},
    Branch,
    Transfer,
    Bitwise,
    Nop
}

impl Alu {
    pub fn decode(raw_instruction: u16) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & 0xFC00;
        let instruction = match opcode {
            0x0C00 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::Arithmetic{op: opcode >> 10, rd, rr}
                },
            _ => Instruction::Branch
        };
        instruction
    }

    pub fn execute(instruction: &Instruction, mut register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        match instruction {
            Instruction::Nop => (),
            Instruction::Arithmetic{op, rd, rr} => Alu::execute_arithmetic(*op, *rd, *rr, &mut register_bank),
            _ => unimplemented!()
        }
    }

    pub fn execute_arithmetic(op: u16, rd: u8, rr: u8, register_bank: &mut RegisterBank) {
        let rdu = rd as usize;
        let rru = rr as usize;
        match op {
            0x3 => register_bank.registers[rdu] = 
                register_bank.registers[rdu] + register_bank.registers[rru],
            0x7 => {
                let carry = if register_bank.get_carry() {1} else {0};
                register_bank.registers[rdu] = 
                register_bank.registers[rdu] + register_bank.registers[rru] + carry
            },
            _ => unimplemented!()
        }
    }
}
