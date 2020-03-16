#![allow(clippy::verbose_bit_mask)]
/// Controller module, which contains a memory bank, registers and an
/// ALU for instruction execution.
pub mod mcu;
/// MCU Factory, creates MCUs by name
pub mod mcu_factory;
/// Memory bank, the main memory of the microcontroller
pub mod memory_bank;
/// Register bank, holds general purpose registers, program counter, and flags
pub mod register_bank;
/// Instruction decoder. Parses words fetched in the memory bank into structs
/// that the ALU can execute.
pub mod decoder;
/// Arithmetic-Logic unit, executes instructions decoded by `Decoder`. Instructions
/// need register bank to read operands and store results. Some instructions
/// (like load-store ones) need a memory bank too.
mod alu;
/// Implementation of fmt::Display
mod display_instruction;

type RawInstruction = u16;

#[derive(Debug,Clone,Copy)]
pub enum PointerRegister{
    X, Y, Z
}

#[derive(Debug)]
/// Decoded instructions
pub enum Instruction {
    Branch {op: u8, test_set: bool, offset: i8},
    CallJmp {is_call: bool, relative: bool, address: u16},
    InOut {is_in: bool, reg: u8, address: u8},
    Nop,
    OneRegOp {rd: u8, op: u8},
    PushPop {is_pop: bool, reg: u8},
    RegConstOp {op: RawInstruction, rd: u8, constant: u8},
    Ret {is_interrupt: bool},
    TransferIndirect {is_load: bool, pointer: PointerRegister, dest: u8, offset: u8},
    // For pre increment and post decrement transfers
    TransferChangePointer {is_load: bool, pointer: PointerRegister, dest: u8, post_inc: bool}, 
    TwoRegOp {op: RawInstruction, rd: u8, rr: u8},
    Unsupported {instruction: RawInstruction}
}