use super::alu;
use super::Instruction;
use super::PointerRegister;
use super::RawInstruction;

/// # Decoder
///
/// Decodes and executes instructions
pub struct Decoder;

impl Decoder {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {
            return Instruction::Nop;
        };
        let opcode = raw_instruction >> 12; // 4 most sig. bits
        match opcode {
            0x0 | 0x1 | 0x2 => {
                if raw_instruction & 0xFC00 == 0 {
                    decode_misc_mult_op(raw_instruction)
                } else {
                    decode_two_reg_op(raw_instruction)
                }
            }
            0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0xE => decode_reg_const_op(raw_instruction),
            0x8 | 0xA => {
                // LDD / STD
                decode_load_store(raw_instruction)
            }
            0x9 => {
                // Misc operations
                decode_misc_op(raw_instruction)
            }
            0xB => decode_in_out(raw_instruction),
            0xC | 0xD => {
                let is_call = opcode == 0xD;
                let offset = raw_instruction & 0xFFF;
                Instruction::CallJmp {
                    is_call,
                    relative: true,
                    address: offset,
                }
            }
            0xF => decode_branch_skip_status_op(raw_instruction),
            _ => {
                unreachable!()
            }
        }
    }
}

fn decode_misc_mult_op(raw_instruction: RawInstruction) -> Instruction {
    let sub_op = raw_instruction & 0x0300;
    let rd = ((raw_instruction & 0x0F0) >> 4) as u8;
    let rr = (raw_instruction & 0x000F) as u8;
    match sub_op {
        0x0100 => Instruction::TwoRegOp {
            op: alu::MOVW_OP,
            rd: rd * 2,
            rr: rr * 2,
        },
        0x0200 => Instruction::TwoRegOp {
            op: alu::MULS_OP,
            rd: rd + 16,
            rr: rr + 16,
        },
        0x0300 => match (rd < 8, rr < 8) {
            (true, true) => Instruction::TwoRegOp {
                op: alu::MULSU_OP,
                rd: rd + 16,
                rr: rr + 16,
            },
            (true, false) => Instruction::TwoRegOp {
                op: alu::FMUL_OP,
                rd: rd + 16,
                rr: rr + 8,
            },
            (false, true) => Instruction::TwoRegOp {
                op: alu::FMULS_OP,
                rd: rd + 8,
                rr: rr + 16,
            },
            (false, false) => Instruction::TwoRegOp {
                op: alu::FMULSU_OP,
                rd: rd + 8,
                rr: rr + 8,
            },
        },
        _ => unreachable!(),
    }
}

fn decode_two_reg_op(raw_instruction: RawInstruction) -> Instruction {
    let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
    let mut rr = (raw_instruction & 0x000F) as u8;
    if raw_instruction & 0x0200 != 0 {
        rr += 16
    }
    Instruction::TwoRegOp {
        op: raw_instruction >> 10,
        rd,
        rr,
    }
}

fn decode_reg_const_op(raw_instruction: RawInstruction) -> Instruction {
    let rd = ((raw_instruction & 0x00F0) >> 4) as u8;
    let constant_upper = ((raw_instruction & 0x0F00) >> 4) as u8;
    let constant_lower = (raw_instruction & 0x000F) as u8;
    let constant = constant_upper + constant_lower;
    Instruction::RegConstOp {
        op: raw_instruction >> 12,
        rd,
        constant,
    }
}

fn decode_load_store(raw_instruction: RawInstruction) -> Instruction {
    let is_load = raw_instruction & 0x0200 == 0;
    let pointer = if raw_instruction & 0x0008 == 0 {
        PointerRegister::Z
    } else {
        PointerRegister::Y
    };
    let dest = ((raw_instruction & 0x01F0) >> 4) as u8;
    let (offset_lo, offset_mid, offset_hi) = (
        raw_instruction & 0x7,
        (raw_instruction & 0x0C00) >> 7,
        (raw_instruction & 0x2000) >> 8,
    );
    let offset = offset_lo + offset_mid + offset_hi;
    Instruction::TransferIndirect {
        is_load,
        pointer,
        dest,
        offset: offset as u8,
    }
}

// These instructions start with 0x9
fn decode_misc_op(raw_instruction: RawInstruction) -> Instruction {
    match raw_instruction & 0x0F00 {
        0 | 0x0100 | 0x0200 | 0x0300 => {
            let dest = ((raw_instruction & 0x01F0) >> 4) as u8;
            let is_load = raw_instruction & 0x0200 == 0;
            match raw_instruction & 0xF {
                0x1 => {
                    // Z post incremented
                    let pointer = PointerRegister::Z;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: true,
                    }
                }
                0x2 => {
                    // Z pre decremented
                    let pointer = PointerRegister::Z;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: false,
                    }
                }
                0x9 => {
                    // Y post incremented
                    let pointer = PointerRegister::Y;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: true,
                    }
                }
                0xA => {
                    // Y pre decremented
                    let pointer = PointerRegister::Y;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: false,
                    }
                }
                0xC => {
                    let pointer = PointerRegister::X;
                    Instruction::TransferIndirect {
                        is_load,
                        pointer,
                        dest,
                        offset: 0,
                    }
                }
                0xD => {
                    let pointer = PointerRegister::X;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: true,
                    }
                }
                0xE => {
                    let pointer = PointerRegister::X;
                    Instruction::TransferChangePointer {
                        is_load,
                        pointer,
                        dest,
                        post_inc: false,
                    }
                }
                0xF => Instruction::PushPop {
                    is_pop: is_load,
                    reg: dest,
                },
                _ => Instruction::Unsupported {
                    instruction: raw_instruction,
                },
            }
        }
        0x0400 => {
            let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
            let op = (raw_instruction & 0xF) as u8;
            Instruction::OneRegOp { rd, op }
        }
        0x0500 => {
            let op = (raw_instruction & 0xF) as u8;
            if op == 0x8 {
                let sub_op = ((raw_instruction & 0x00F0) >> 4) as u8;
                match sub_op {
                    0x2..=0x7 | 0xB => Instruction::Unsupported {
                        instruction: raw_instruction,
                    },
                    _ => Instruction::ZeroRegOp { op: sub_op },
                }
            } else {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                Instruction::OneRegOp { rd, op }
            }
        }
        0x0600 | 0x0700 => {
            let op = (raw_instruction & 0xFF00) >> 8;
            let rd = ((raw_instruction & 0x30) >> 4) as u8;
            let constant = (((raw_instruction & 0xC0) >> 2) + raw_instruction & 0xF) as u8;
            Instruction::RegConstOp { op, rd, constant }
        }
        0x0800 | 0x0A00 => {
            let sub_op = ((raw_instruction & 0x0F00) >> 8) as u8;
            let address = ((raw_instruction & 0x0F8) >> 3) as u8;
            let bit = (raw_instruction & 7) as u8;
            Instruction::BitManipOp {
                address,
                bit,
                set: sub_op == 0xA,
            }
        }
        0x0900 | 0x0B00 => {
            let sub_op = ((raw_instruction & 0x0F00) >> 8) as u8;
            let address = ((raw_instruction & 0x0F8) >> 3) as u8;
            let bit = (raw_instruction & 7) as u8;
            Instruction::SkipOp {
                address,
                bit,
                set: sub_op == 0xB,
            }
        }
        _ => Instruction::Unsupported {
            instruction: raw_instruction,
        },
    } // end match raw_instruction & 0x0F00
}

fn decode_in_out(raw_instruction: RawInstruction) -> Instruction {
    let is_in = raw_instruction & 0x0800 == 0;
    let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
    let address_low = (raw_instruction & 0x000F) as u8;
    let address_hi = ((raw_instruction & 0x0600) >> 5) as u8;
    let address = address_hi + address_low;
    Instruction::InOut {
        is_in,
        reg,
        address,
    }
}

fn decode_branch_skip_status_op(raw_instruction: RawInstruction) -> Instruction {
    if raw_instruction & 0x0800 == 0 {
        let op = (raw_instruction & 0x0007) as u8;
        let u_offset = ((raw_instruction & 0x03F8) >> 3) as u8;
        let offset = if u_offset < 0x40 {
            u_offset as i8
        } else {
            u_offset.wrapping_sub(128) as i8
        };
        let test_set = raw_instruction & 0x0400 == 0;
        Instruction::Branch {
            op,
            test_set,
            offset,
        }
    } else {
        Instruction::Unsupported {
            instruction: raw_instruction,
        }
    }
}
