extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

/// Tests simple substract immediate instruction
///
/// SUBI opcode: `0101 KKKK dddd KKKK`
/// where `dddd` is a register between 16 and 31
/// 150=0x96, 100 = 0x64 -> 0x96 - 0x64 = 0x32 = 50
/// subi r18, 100 -> 0101 0110 0010 0100 -> 5624
///
/// Remember AVR is little endian!
#[test]
fn test_subi() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x96);
    let memory_data = vec![0x24, 0x56];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x32);
}

/// Tests simple substract immediate instruction register with 0 minus constant 0
/// subi r18, 0 -> 0101 0000 0010 0000 -> 5020
#[test]
fn test_subi_zeros() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x00);
    let memory_data = vec![0x20, 0x50];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x0);
}

/// Tests simple substract immediate instruction, register with 0 minus constant 1
/// subi r18, 0 -> 0101 0000 0010 0000 -> 5020
#[test]
fn test_subi_one_zero() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x01);
    let memory_data = vec![0x20, 0x50];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x1);
}

/// Tests simple substract immediate instruction, register with 0 minus constant 1
/// subi r18, 1 -> 0101 0000 0010 0001 -> 5021
#[test]
fn test_subi_zero_one() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0x21, 0x50];
    mcu.load_program_memory(&memory_data);
    mcu.set_register(18, 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(18), 0xFF);
    let flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(!flags.zero);
    assert!(flags.neg);
    assert!(!flags.over);
    assert!(flags.sign);
    assert!(flags.half);
}

/// Tests simple substract instruction 3 minus -4
/// subi r18, 100 -> 0101 1111 0010 1100 -> 5F2C
#[test]
fn test_subi_pos_minus_neg() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0x2C, 0x5F];
    mcu.load_program_memory(&memory_data);
    mcu.set_register(18, 0x03);
    mcu.step();
    assert_eq!(mcu.get_register(18), 0x7);
    let flags = mcu.get_flags();
    assert!(!flags.zero);
}

/// Tests simple substract instruction 0 minus 8
/// subi r18, 100 -> 0101 0000 0010 1000 -> 5028
#[test]
fn test_subi_zero_minus_half() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0x28, 0x50];
    mcu.load_program_memory(&memory_data);
    mcu.set_register(18, 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(18), 0xf8);
    let flags = mcu.get_flags();
    assert!(!flags.zero);
}

/// Tests simple substract immediate instruction, with carry
///
/// SBCI opcode: `0100 KKKK dddd KKKK`
/// where `dddd` is a register between 16 and 31
/// 150=0x96, 100 = 0x64 -> 0x96 - 0x64 = 0x32 = 50
/// sbci r18, 100 -> 0100 0110 0010 0100 -> 4624
///
/// Remember AVR is little endian!
#[test]
fn test_sbci() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x96);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    let memory_data = vec![0x24, 0x46];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x31);
}

/// Tests simple substract immediate instruction register with 0 minus constant 0
/// Carry flag on and off
/// sbci r18, 0 -> 0100 0000 0010 0000 -> 4020
#[test]
fn test_sbci_zeros() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x00);
    mcu.set_register(19, 0x00);
    let mut flags = mcu.get_flags();
    flags.carry = false;
    mcu.set_flags(flags);
    let memory_data = vec![0x20, 0x40, 0x30, 0x40];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_register(18), 0x0);
    assert_eq!(mcu.get_register(19), 0xFF);
}

/// Tests simple substract immediate instruction register with 1 minus constant 0
/// Carry flag on and off
/// sbci r18, 0 -> 0100 0000 0010 0000 -> 4021
#[test]
fn test_sbci_one_zero() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x01);
    mcu.set_register(19, 0x01);
    let mut flags = mcu.get_flags();
    flags.carry = false;
    let memory_data = vec![0x20, 0x40, 0x30, 0x40];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x1);
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_register(19), 0x0);
}

/// Tests simple substract immediate instruction, register with 0 minus constant 1
/// sbci r18, 1 -> 0100 0000 0010 0001 -> 4021
#[test]
fn test_sbci_zero_one() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x0);
    mcu.set_register(19, 0x0);
    let mut flags = mcu.get_flags();
    flags.carry = false;
    let memory_data = vec![0x21, 0x40, 0x31, 0x40];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0xFF);
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_register(19), 0xFE);
}

/// Tests simple substract instruction 3 minus -4
/// sbci r18, -4 -> 0100 1111 0010 1100 -> 4F2C
#[test]
fn test_sbci_pos_minus_neg() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x3);
    mcu.set_register(19, 0x3);
    let mut flags = mcu.get_flags();
    flags.carry = false;
    let memory_data = vec![0x2C, 0x4F, 0x3C, 0x4F];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0x07);
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_register(19), 0x06);
}

/// Tests simple substract instruction 0 minus 8
/// sbci r18, 8 -> 0100 0000 0010 1000 -> 4028
#[test]
fn test_sbci_zero_minus_half() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(18, 0x0);
    mcu.set_register(19, 0x0);
    let mut flags = mcu.get_flags();
    flags.carry = false;
    let memory_data = vec![0x28, 0x40, 0x38, 0x40];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(18), 0xF8);
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_register(19), 0xF7);
}
