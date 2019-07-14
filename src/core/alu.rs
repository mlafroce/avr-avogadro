use super::register_bank::RegisterBank;
use super::memory_bank::MemoryBank;
/// # ALU
///
/// Decodes and executes instructions
pub struct Alu;

const RAW_OPCODE_MASK: u16 = 0xF000;

#[derive(Debug)]
pub enum Instruction {
    TwoOperand {op: u16, rd: u8, rr: u8},
    Branch,
    Transfer,
    Bitwise,
    Nop
}

impl Alu {
    pub fn decode(raw_instruction: u16) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & RAW_OPCODE_MASK;
        let instruction = match opcode {
            0x0000 | 0x1000 | 0x2000 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::TwoOperand{op: raw_instruction >> 10, rd, rr}
                },
            _ => Instruction::Branch
        };
        instruction
    }

    pub fn execute(instruction: &Instruction, mut register_bank: &mut RegisterBank, memory_bank: &mut MemoryBank) {
        match instruction {
            Instruction::Nop => (),
            Instruction::TwoOperand{op, rd, rr} => Alu::execute_arithmetic(*op, *rd, *rr, &mut register_bank),
            _ => unimplemented!()
        }
    }

    pub fn execute_arithmetic(op: u16, rd: u8, rr: u8, register_bank: &mut RegisterBank) {
        let rdu = rd as usize;
        let rru = rr as usize;
        match op {
            0x3 => Alu::add(rdu, rru, register_bank, 0),
            0x7 => {
                let carry = if register_bank.get_flags().carry {1} else {0};
                Alu::add(rdu, rru, register_bank, carry)
            },
            _ => unimplemented!()
        }
    }

    fn add(rdu: usize, rru: usize, register_bank: &mut RegisterBank, carry: u8) {
        let sum: u16 = register_bank.registers[rdu] as u16 +
            register_bank.registers[rru] as u16 + carry as u16;
        register_bank.registers[rdu] = sum as u8;
        let mut flags = register_bank.get_flags();
        flags.carry = sum > 0xFF;
        flags.zero = (sum & 0xFF) == 0;
        register_bank.set_flags(flags);
    }
}
