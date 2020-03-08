extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

/// Tests simple and instruction
///
/// AND opcode: 0010 00rd dddd rrrr
/// add r1, r2 -> 0010 0000 0001 0010 -> 2012
///
/// Remember AVR is little endian!
#[test]
fn test_and() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0xC5);
    mcu.set_register(2, 0x95);
    let memory_data = vec![0x12, 0x20];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x85);
}

/// Tests simple add instruction where rd = 0
///
/// AND opcode: 0010 00rd dddd rrrr
/// and r1, r2 -> 0010 0000 0001 0010 -> 2012
#[test]
fn test_and_zero() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0x00);
    mcu.set_register(2, 0xF0);
    let memory_data = vec![0x12, 0x20];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x00);
    let flags = mcu.get_flags();
    assert!(flags.zero);
}

#[test]
/// ANDI opcode: 0111 kkkk dddd kkkk
/// or r16, 0x95 -> 0111 1001 0000 0101 -> 7905
fn test_andi() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(16, 0xC5);
    let memory_data = vec![0x05, 0x79];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(16), 0x85);
}

/// Tests simple exc. or instruction
///
/// OR opcode: 0010 01rd dddd rrrr
/// or r1, r2 -> 0010 0100 0001 0010 -> 2412
#[test]
fn test_eor() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0xC5);
    mcu.set_register(2, 0x95); //// C^9 -> 1100 ^ 1001 -> 0101
    let memory_data = vec![0x12, 0x24];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x50);
}

#[test]
/// Tests simple exc. or instruction where rd = 0
fn test_eor_zero() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0x00);
    mcu.set_register(2, 0xF0);
    let memory_data = vec![0x12, 0x24];
    mcu.load_program_memory(&memory_data);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0xF0);
}

/// Tests simple or instruction
///
/// OR opcode: 0010 10rd dddd rrrr
/// or r1, r2 -> 0010 1000 0001 0010 -> 2812
#[test]
fn test_or() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0xC5);
    mcu.set_register(2, 0x95);
    let memory_data = vec![0x12, 0x28];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0xD5);
}

#[test]
/// Tests simple or instruction where rd = 0
fn test_or_zero() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0x00);
    mcu.set_register(2, 0xF0);
    let memory_data = vec![0x12, 0x28];
    mcu.load_program_memory(&memory_data);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0xF0);
}

#[test]
/// ORI opcode: 0110 kkkk dddd kkkk
/// or r16, 0x95 -> 0110 1001 0000 0101 -> 7905
fn test_ori() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(16, 0xC5);
    let memory_data = vec![0x05, 0x69];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(16), 0xD5);
}
/// Tests simple mov instruction
///
/// MOV opcode: 0010 11rd dddd rrrr
/// mov r1, r2 -> 0010 1100 0001 0010 -> 2C12
#[test]
fn test_mov() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0xC5);
    mcu.set_register(2, 0x95);
    let memory_data = vec![0x12, 0x2C];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x95);
}

#[test]
/// Tests simple mov instruction where rr = 0
fn test_mov_zero() {
    let mut mcu = McuFactory::create("attiny85");
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.set_register(1, 0x00);
    mcu.set_register(2, 0xF0);
    let memory_data = vec![0x21, 0x2C];
    mcu.load_program_memory(&memory_data);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(2), 0x00);
}
