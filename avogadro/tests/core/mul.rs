extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

// #[test]
/// Tests simple mul instruction
///
/// MUL opcode: 1001 11rd dddd rrrr
/// add r1, r2 -> 0000 1100 0001 0002 -> 0C12
///
/// Remember AVR is little endian!
fn test_mul() {}
