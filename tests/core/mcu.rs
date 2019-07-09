extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;
use avr_avogadro::core::memory_bank::MemoryBank;

#[test]
/// Tests símple Nop instruction
/// Program counter should advance
fn test_step() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0, 0];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
}

#[test]
/// Tests símple add instruction
///
/// ADD opcode: 0000 11rd dddd rrrr
/// add r1, r2 -> 0000 1100 0001 0002 -> 0C12
///
/// Remember AVR is little endian!
fn test_sum() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 4);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x0C];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x09);
}