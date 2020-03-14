use super::Instruction;
use super::PointerRegister;
use super::RawInstruction;
use std::fmt;

/// # Decoder
///
/// Decodes and executes instructions
pub struct Decoder;

const MAIN_OPCODE_MASK: RawInstruction = 0xF000;

impl Decoder {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & MAIN_OPCODE_MASK;  // 4 most sig. bits
        match opcode {
            0x0000 | 0x1000 | 0x2000 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::TwoRegOp{op: raw_instruction >> 10, rd, rr}
                },
            0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 | 0xE000 => {
                let rd = ((raw_instruction & 0x00F0) >> 4) as u8;
                let constant_upper = ((raw_instruction & 0x0F00) >> 4) as u8;
                let constant_lower = (raw_instruction & 0x000F) as u8;
                let constant = constant_upper + constant_lower;
                Instruction::RegConstOp{op: raw_instruction >> 12, rd, constant}
            },
            0x8000 | 0xA000 => { // LDD / STD
                let is_load = raw_instruction & 0x0200 == 0;
                let base_reg = if raw_instruction & 0x0008 == 0 {PointerRegister::Z} else {PointerRegister::Y};
                let dest = ((raw_instruction & 0x01F0) >> 4) as u8;
                let (offset_lo, offset_mid, offset_hi) = (raw_instruction & 0x7,
                    (raw_instruction & 0x0C00) >> 7, (raw_instruction & 0x2000) >> 8);
                let offset =  offset_lo + offset_mid + offset_hi;
                Instruction::TransferIndirect{is_load, base_reg, dest, offset: offset as u8}
            },
            0x9000 => { // Misc operations
                match raw_instruction {
                    0x9508 => Instruction::Ret{is_interrupt: false},
                    0x9518 => Instruction::Ret{is_interrupt: true},
                    _ => {
                        match raw_instruction & 0x0F00 {
                            0 | 0x0100 | 0x0200 | 0x0300 => {
                                match raw_instruction & 0xF {
                                    0xF => {
                                        let is_pop = raw_instruction & 0x0200 == 0;
                                        let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                                        Instruction::PushPop{is_pop, reg}
                                    },
                                    0xD => {
                                        let is_load = raw_instruction & 0x0200 == 0;
                                        let dest = ((raw_instruction & 0x01F0) >> 4) as u8;
                                        let base_reg = PointerRegister::X;
                                        Instruction::TransferIndirect{is_load, base_reg, dest, offset: 0}
                                    }
                                    _ => Instruction::Unsupported { instruction: raw_instruction }
                                }
                            },
                            0x0400 | 0x0500 => {
                                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                                let op = (raw_instruction & 0xF) as u8;
                                Instruction::OneRegOp { rd, op }
                            }
                            0x0600 | 0x0700 => {
                                let op = (raw_instruction & 0xFF00) >> 8;
                                let rd = ((raw_instruction & 0x30) >> 4) as u8;
                                let constant = (((raw_instruction & 0xC0) >> 2) + raw_instruction & 0xF) as u8;
                                Instruction::RegConstOp{op, rd, constant}
                            },
                            _ => Instruction::Unsupported { instruction: raw_instruction }
                        } // end match raw_instruction & 0x0F00
                    }
                }
            },
            0xB000 => {
                let is_in = raw_instruction & 0x0800 == 0;
                let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                let address_low = (raw_instruction & 0x000F) as u8;
                let address_hi = ((raw_instruction & 0x0600) >> 5) as u8;
                let address = address_hi + address_low;
                Instruction::InOut { is_in, reg, address }
            },
            0xC000 | 0xD000 => {
                let is_call = opcode == 0xD000;
                let offset = raw_instruction & 0xFFF;
                Instruction::CallJmp { is_call, relative: true, address: offset }
            },
            0xF000 => {
                if raw_instruction & 0x0800 == 0 {
                    let op = (raw_instruction & 0x0007) as u8;
                    let u_offset = ((raw_instruction & 0x03F8) >> 3) as u8;
                    let offset = if u_offset < 64 { u_offset as i8 }
                        else { u_offset.wrapping_sub(128) as i8 };
                    let test_set = raw_instruction & 0x0400 == 0;
                    Instruction::Branch { op, test_set, offset }
                } else {
                    Instruction::Unsupported { instruction: raw_instruction }
                }
            },
            _ => {
                unreachable!()
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Branch { op, test_set, offset }
                => display_branch(f, *op, *test_set, *offset),
            Instruction::CallJmp { is_call, relative, address } => {
                let r_str = if *relative { "r" } else { "" };
                let op_str = if *is_call { "call" } else { "jmp" };
                write!(f, "{}{}\t0x{:x}", r_str, op_str, *address)
            },
            Instruction::InOut {is_in, reg, address } => {
                let op_str = if *is_in { "in" } else { "out" };
                write!(f, "{}\tr{} 0x{:x}", op_str, *reg, *address)
            },
            Instruction::Nop => write!(f, "nop"),
            Instruction::OneRegOp {op, rd} => write!(f, "Parsed but unsupported instruction"),
            Instruction::PushPop { is_pop, reg } => {
                let op_str = if *is_pop { "pop" } else { "push" };
                write!(f, "{}\tr{}", op_str, *reg)
            },
            Instruction::RegConstOp {op, rd, constant } => 
                display_arith_costant(f, *op, *rd, *constant),
            Instruction::Ret {is_interrupt} => {
                let op_str = if *is_interrupt { "reti" } else { "ret" };
                write!(f, "{}", op_str)
            }
            Instruction::TransferIndirect { is_load, base_reg, dest, offset } =>  {
                let op_str = if *is_load { "ldd" } else { "std" };
                let base_reg_str = match *base_reg {
                    PointerRegister::X => "X",
                    PointerRegister::Y => "Y",
                    PointerRegister::Z => "Z"
                };
                write!(f, "{}\t{}+{}, r{}", op_str, base_reg_str, offset, dest)
            },
            Instruction::TwoRegOp { op, rd, rr } => 
                display_two_reg_op(f, *op, *rd, *rr),
            Instruction::Unsupported { instruction } => 
                write!(f, "Unsupported instruction: {:x}", *instruction)
        }
    }
}

fn display_two_reg_op(f: &mut fmt::Formatter<'_>,
    op: RawInstruction, rd: u8, rr: u8) -> fmt::Result {
    match op {
        0x1 => write!(f, "cpc\tr{}, r{}", rd, rr),
        0x2 => write!(f, "sbc\tr{}, r{}", rd, rr),
        0x3 => write!(f, "add\tr{}, r{}", rd, rr),
        0x4 => write!(f, "cpse\tr{}, r{}", rd, rr),
        0x5 => write!(f, "cp\tr{}, r{}", rd, rr),
        0x6 => write!(f, "sub\tr{}, r{}", rd, rr),
        0x7 => write!(f, "adc\tr{}, r{}", rd, rr),
        0x8 => write!(f, "and\tr{}, r{}", rd, rr),
        0x9 => write!(f, "eor\tr{}, r{}", rd, rr),
        0xA => write!(f, "or\tr{}, r{}", rd, rr),
        0xB ..=
        0xF => write!(f, "mov\tr{}, r{}", rd, rr),
        _ => unreachable!()
    }
}

fn display_arith_costant(f: &mut fmt::Formatter<'_>,
    op: RawInstruction, rd: u8, constant: u8) -> fmt::Result {
    let real_rd = rd + 16;
    match op {
        0x3 => write!(f, "cpi\tr{}, 0x{:x}", real_rd, constant),
        0x4 => write!(f, "sbci\tr{}, 0x{:x}", real_rd, constant),
        0x5 => write!(f, "subi\tr{}, 0x{:x}", real_rd, constant),
        0x6 => write!(f, "ori\tr{}, 0x{:x}", real_rd, constant),
        0x7 => write!(f, "andi\tr{}, 0x{:x}", real_rd, constant),
        // ldi is technically a transfer instruction
        0xE => write!(f, "ldi\tr{}, 0x{:x}", real_rd, constant),
        0x96 =>write!(f, "adiw\tr{}, 0x{:x}", real_rd, constant),
        0x97 =>write!(f, "sbiw\tr{}, 0x{:x}", real_rd, constant),
        _ => unreachable!()
    }
}

fn display_branch(f: &mut fmt::Formatter<'_>,
    op: u8, test_set: bool, offset: i8) -> fmt::Result {
    let display_offset = offset * 2;
    if test_set {
        match op {
            0x0 => write!(f, "brcs\t.{:#}", display_offset),
            0x1 => write!(f, "breq\t.{:#}", display_offset),
            0x2 => write!(f, "brmi\t.{:#}", display_offset),
            0x3 => write!(f, "brvs\t.{:#}", display_offset),
            0x4 => write!(f, "brlt\t.{:#}", display_offset),
            0x5 => write!(f, "brhs\t.{:#}", display_offset),
            0x6 => write!(f, "brts\t.{:#}", display_offset),
            0x7 => write!(f, "brie\t.{:#}", display_offset),
            _ => unreachable!()
        }
    } else {
        match op {
            0x0 => write!(f, "brcc\t.{:#}", display_offset),
            0x1 => write!(f, "brne\t.{:#}", display_offset),
            0x2 => write!(f, "brpl\t.{:#}", display_offset),
            0x3 => write!(f, "brvc\t.{:#}", display_offset),
            0x4 => write!(f, "brge\t.{:#}", display_offset),
            0x5 => write!(f, "brhc\t.{:#}", display_offset),
            0x6 => write!(f, "brtc\t.{:#}", display_offset),
            0x7 => write!(f, "brid\t.{:#}", display_offset),
            _ => unreachable!()
        }
    }
}