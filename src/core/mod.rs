#![allow(clippy::verbose_bit_mask)]
/// Controller module, which contains a memory bank, registers and an
/// ALU for instruction execution.
pub mod mcu;
/// Memory bank, the main memory of the microcontroller
pub mod memory_bank;
/// Register bank, holds general purpose registers, program counter, and flags
pub mod register_bank;
/// Instruction decoder. Parses words fetched in the memory bank into structs
/// that the ALU can execute.
mod decoder;
/// Arithmetic-Logic unit, executes instructions decoded by `Decoder`. Instructions
/// need register bank to read operands and store results. Some instructions
/// (like load-store ones) need a memory bank too.
mod alu;

type RawInstruction = u16;

#[derive(Debug)]
/// Decoded instructions
pub enum Instruction {
    OneRegOp,
    TwoRegOp {op: RawInstruction, rd: u8, rr: u8},
    RegConstOp {op: RawInstruction, rd: u8, constant: u8},
    Transfer {is_load: bool, reg: u8, opcode: u8},
    CallJmp {is_call: bool, relative: bool, address: u16},
//    Bitwise,
    Nop,
    Unsupported {instruction: RawInstruction}
}
