use super::Instruction;
use super::PointerRegister;
use super::RawInstruction;
use super::alu;
use std::fmt;

impl fmt::Display for Instruction {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Branch { op, test_set, offset }
                => display_branch(f, *op, *test_set, *offset),
            Instruction::CallJmp { is_call, relative, address } => {
                let r_str = if *relative { "r" } else { "" };
                let op_str = if *is_call { "call" } else { "jmp" };
                write!(f, "{}{}\t0x{:02X}", r_str, op_str, *address)
            },
            Instruction::InOut {is_in, reg, address } => {
                let op_str = if *is_in { "in" } else { "out" };
                write!(f, "{}\tr{} 0x{:02X}", op_str, *reg, *address)
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
                write!(f, "Unsupported instruction: {:02X}", *instruction)
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
        alu::MOVW_OP => write!(f, "movw\tr{}, r{}", rd, rr),
        alu::MULS_OP => write!(f, "muls\tr{}, r{}", rd, rr),
        alu::MULSU_OP => write!(f, "mulsu\tr{}, r{}", rd, rr),
        alu::FMUL_OP => write!(f, "fmul\tr{}, r{}", rd, rr),
        alu::FMULS_OP => write!(f, "fmuls\tr{}, r{}", rd, rr),
        alu::FMULSU_OP => write!(f, "fmulsu\tr{}, r{}", rd, rr),
        _ => unreachable!()
    }
}

fn display_arith_costant(f: &mut fmt::Formatter<'_>,
    op: RawInstruction, rd: u8, constant: u8) -> fmt::Result {
    let real_rd = rd + 16;
    match op {
        0x3 => write!(f, "cpi\tr{}, 0x{:02X}", real_rd, constant),
        0x4 => write!(f, "sbci\tr{}, 0x{:02X}", real_rd, constant),
        0x5 => write!(f, "subi\tr{}, 0x{:02X}", real_rd, constant),
        0x6 => write!(f, "ori\tr{}, 0x{:02X}", real_rd, constant),
        0x7 => write!(f, "andi\tr{}, 0x{:02X}", real_rd, constant),
        // ldi is technically a transfer instruction
        0xE => write!(f, "ldi\tr{}, 0x{:02X}", real_rd, constant),
        0x96 =>write!(f, "adiw\tr{}, 0x{:02X}", real_rd, constant),
        0x97 =>write!(f, "sbiw\tr{}, 0x{:02X}", real_rd, constant),
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