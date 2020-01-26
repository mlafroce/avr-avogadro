extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

/// Tests simple add instruction
///
/// SUB opcode: 0000 10rd dddd rrrr
/// add r1, r2 -> 0000 1000 0001 0002 -> 0812
///
/// Remember AVR is little endian!
#[test]
fn test_sub() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 9);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x04);
}

/// Tests simple substract instruction between zeros
#[test]
fn test_sub_zeros() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);
    mcu.set_register(1, 0x0);
    mcu.set_register(2, 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(1), 0x0);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

/// Tests simple substract instruction 1 minus 0
#[test]
fn test_sub_one_zero() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);
    mcu.set_register(1, 0x1);
    mcu.set_register(2, 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(1), 0x1);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(!flags.zero);
}

/// Tests simple substract instruction 1 minus 0
#[test]
fn test_sub_neg_minus_pos() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);
    mcu.set_register(1, 0x03);
    mcu.set_register(2, 0xFC); // -4
    mcu.step();
    assert_eq!(mcu.get_register(1), 0x7);
    let flags = mcu.get_flags();
    assert!(!flags.zero);
}

/// Tests simple substract instruction 1 minus 0
#[test]
fn test_sub_zero_minus_half() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);
    mcu.set_register(1, 0x0);
    mcu.set_register(2, 0x8);
    mcu.step();
    assert_eq!(mcu.get_register(1), 0xf8);
    let flags = mcu.get_flags();
    assert!(!flags.zero);
}
