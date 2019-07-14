extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;
use avr_avogadro::core::memory_bank::MemoryBank;

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

#[test]
/// Tests símple add instruction which results in zero and carry flag on
fn test_sum_flags() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 0x78);
    mcu.set_register(2, 0x88);
    // 0x78 + 0x88 = 0x100
    let memory_data = vec![0x12, 0x0C];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    let flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(flags.zero);
}
