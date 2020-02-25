use super::Instruction;
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
                let is_base_z = raw_instruction & 0x0008 == 0;
                let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                let (offset_lo, offset_mid, offset_hi) = (raw_instruction & 0x7,
                    (raw_instruction & 0x0C00) >> 7, (raw_instruction & 0x2000) >> 8);
                let offset =  offset_lo + offset_mid + offset_hi;
                Instruction::TransferIndirect{is_load, is_base_z, reg, offset: offset as u8}
            },
            0x9000 => { // One register operations?
                match raw_instruction {
                    0x9508 => Instruction::Ret{is_interrupt: false},
                    0x9518 => Instruction::Ret{is_interrupt: true},
                    _ => {
                        match raw_instruction & 0x0E00 {
                            0 | 0x0200 => {
                                let is_pop = raw_instruction & 0x0200 == 0;
                                let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                                if raw_instruction & 0x000F == 0xF {
                                    Instruction::PushPop {is_pop, reg}
                                } else {
                                    Instruction::OneRegOp
                                }
                            }
                            _ => Instruction::OneRegOp
                        }
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
                write!(f, "{}{}, 0x{}", r_str, op_str, *address)
            },
            Instruction::InOut {is_in, reg, address } => {
                let op_str = if *is_in { "in" } else { "out" };
                write!(f, "{} r{} 0x{:x}", op_str, *reg, *address)
            },
            Instruction::Nop => write!(f, "nop"),
            Instruction::OneRegOp => write!(f, "Parsed but unsupported instruction"),
            Instruction::PushPop { is_pop, reg } => {
                let op_str = if *is_pop { "pop" } else { "push" };
                write!(f, "{} r{}", op_str, *reg)
            },
            Instruction::RegConstOp {op, rd, constant } => 
                display_arith_costant(f, *op, *rd, *constant),
            Instruction::Ret {is_interrupt} => {
                let op_str = if *is_interrupt { "reti" } else { "ret" };
                write!(f, "{}", op_str)
            }
            Instruction::TransferIndirect { is_load, is_base_z, reg, offset } =>  {
                let op_str = if *is_load { "ldd" } else { "std" };
                let base_reg = if *is_base_z { "Z" } else { "Y" };
                write!(f, "{} {}+{}, r{}", op_str, base_reg, offset, reg)
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
    info!("Display {:?}, r{} r{}", op, rd, rr);
    match op {
        0x1 => write!(f, "cpc  r{}, r{}", rd, rr),
        0x2 => write!(f, "sbc  r{}, r{}", rd, rr),
        0x3 => write!(f, "add  r{}, r{}", rd, rr),
        0x4 => write!(f, "cpse r{}, r{}", rd, rr),
        0x5 => write!(f, "cp   r{}, r{}", rd, rr),
        0x6 => write!(f, "sub  r{}, r{}", rd, rr),
        0x7 => write!(f, "adc  r{}, r{}", rd, rr),
        0x8 => write!(f, "and  r{}, r{}", rd, rr),
        0x9 => write!(f, "eor  r{}, r{}", rd, rr),
        0xA => write!(f, "or   r{}, r{}", rd, rr),
        0xB ..=
        0xF => write!(f, "mov  r{}, r{}", rd, rr),
        _ => unreachable!()
    }
}

fn display_arith_costant(f: &mut fmt::Formatter<'_>,
    op: RawInstruction, rd: u8, constant: u8) -> fmt::Result {
    match op {
        0x3 => write!(f, "cpi r{}, 0x{:x}", rd, constant),
        0x4 => write!(f, "sbci r{}, 0x{:x}", rd, constant),
        0x5 => write!(f, "subi r{}, 0x{:x}", rd, constant),
        0x6 => write!(f, "ori  r{}, 0x{:x}", rd, constant),
        0x7 => write!(f, "andi r{}, 0x{:x}", rd, constant),
        // ldi is technically a transfer instruction
        0xE => write!(f, "ldi  r{}, 0x{:x}", rd, constant),
        _ => unreachable!()
    }
}

fn display_branch(f: &mut fmt::Formatter<'_>,
    op: u8, test_set: bool, offset: i8) -> fmt::Result {
    let display_offset = offset * 2;
    if test_set {
        match op {
            0x0 => write!(f, "brcs .{:#}", display_offset),
            0x1 => write!(f, "breq .{:#}", display_offset),
            0x2 => write!(f, "brmi .{:#}", display_offset),
            0x3 => write!(f, "brvs .{:#}", display_offset),
            0x4 => write!(f, "brlt .{:#}", display_offset),
            0x5 => write!(f, "brhs .{:#}", display_offset),
            0x6 => write!(f, "brts .{:#}", display_offset),
            0x7 => write!(f, "brie .{:#}", display_offset),
            _ => unreachable!()
        }
    } else {
        match op {
            0x0 => write!(f, "brcc .{:#}", display_offset),
            0x1 => write!(f, "brne .{:#}", display_offset),
            0x2 => write!(f, "brpl .{:#}", display_offset),
            0x3 => write!(f, "brvc .{:#}", display_offset),
            0x4 => write!(f, "brge .{:#}", display_offset),
            0x5 => write!(f, "brhc .{:#}", display_offset),
            0x6 => write!(f, "brtc .{:#}", display_offset),
            0x7 => write!(f, "brid .{:#}", display_offset),
            _ => unreachable!()
        }
    }
}