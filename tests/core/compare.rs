extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;
use avr_avogadro::core::memory_bank::MemoryBank;

#[test]
/// Tests simple compare instruction
///
/// CP opcode: 0001 01rd dddd rrrr
/// cp r1, r2 -> 0001 0100 0001 0002 -> 1412
///
/// Remember AVR is little endian!
fn test_cp_equals() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 5);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x14];
    mcu.load_memory(&memory_data);
    
    mcu.step();
    assert_eq!(mcu.get_register(1), 0x05);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests simple compare instruction, carry should be ignored
fn test_cp_with_carry_equals() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 5);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x14];
    mcu.load_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    
    mcu.step();
    assert_eq!(mcu.get_register(1), 0x05);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests simple compare instruction. Rd is greater than rr,
/// so carry and zero flags should be zero
fn test_cp_rd_greater() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 200);
    mcu.set_register(2, 127);
    let memory_data = vec![0x12, 0x14];
    mcu.load_memory(&memory_data);
    
    mcu.step();
    assert_eq!(mcu.get_register(1), 200);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(!flags.zero);
}

#[test]
/// Tests simple compare instruction. Rd is less than rr,
/// so carry should be set and zero should be zero
fn test_cp_rd_less() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 150);
    mcu.set_register(2, 200);
    let memory_data = vec![0x12, 0x14];
    mcu.load_memory(&memory_data);
    
    mcu.step();
    assert_eq!(mcu.get_register(1), 150);
    let flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(!flags.zero);
}

#[test]
/// Tests compare with carry instruction
///
/// CP opcode: 0000 01rd dddd rrrr
/// cp r1, r2 -> 0000 0100 0001 0002 -> 0412
fn test_cpc_equals() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 5);
    mcu.set_register(2, 4);
    let memory_data = vec![0x12, 0x04];
    mcu.load_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    mcu.step();
    assert_eq!(mcu.get_register(1), 0x05);
    assert_eq!(mcu.get_register(2), 0x04);
    flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests compare with carry instruction. Rd is 0x00, rr+C should overflow
/// and have the same result
fn test_cpc_rr_overflow() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 0);
    mcu.set_register(2, 255);
    let memory_data = vec![0x12, 0x04];
    mcu.load_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    
    mcu.step();
    flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests simple compare instruction. Rd is less than (rr + C),
/// so carry should be set and zero should be zero
fn test_cpc_rd_less() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 128);
    mcu.set_register(2, 128);
    let memory_data = vec![0x12, 0x04];
    mcu.load_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    
    mcu.step();
    flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(!flags.zero);
}
