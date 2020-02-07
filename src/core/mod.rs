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
    Branch {op: u8, test_set: bool, offset: i8},
    CallJmp {is_call: bool, relative: bool, address: u16},
    InOut {is_in: bool, reg: u8, address: u8},
    Nop,
    OneRegOp,
    PushPop {is_pop: bool, reg: u8},
    RegConstOp {op: RawInstruction, rd: u8, constant: u8},
    Ret {is_interrupt: bool},
    TransferIndirect {is_load: bool, is_base_z: bool, reg: u8, offset: u8},
    TwoRegOp {op: RawInstruction, rd: u8, rr: u8},
    Unsupported {instruction: RawInstruction}
}
