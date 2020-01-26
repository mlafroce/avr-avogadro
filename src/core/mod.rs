#![allow(clippy::verbose_bit_mask)]
/// Controller module, which contains a memory bank, registers and an
/// ALU for instruction execution.
pub mod mcu;
/// Memory bank, the main memory of the microcontroller
pub mod memory_bank;
/// Register bank, holds general purpose registers, program counter, and flags
pub mod register_bank;
/// Arithmetic-Logic unit, decodes instructions and executes them. Instructions
/// need register bank to read operands and store results. Some instructions
/// (like load-store ones) need a memory bank too.
mod alu;