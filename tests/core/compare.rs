extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests simple compare instruction
///
/// CP opcode: 0001 01rd dddd rrrr
/// cp r1, r2 -> 0001 0100 0001 0002 -> 1412
///
/// Remember AVR is little endian!
fn test_cp_equals() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 5);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x14];
    mcu.load_program_memory(&memory_data);

    mcu.step();
    assert_eq!(mcu.get_register(1), 0x05);
    let flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests simple compare instruction, carry should be ignored
fn test_cp_with_carry_equals() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 5);
    mcu.set_register(2, 5);
    let memory_data = vec![0x12, 0x14];
    mcu.load_program_memory(&memory_data);
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
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 200);
    mcu.set_register(2, 127);
    let memory_data = vec![0x12, 0x14];
    mcu.load_program_memory(&memory_data);

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
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 150);
    mcu.set_register(2, 200);
    let memory_data = vec![0x12, 0x14];
    mcu.load_program_memory(&memory_data);

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
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 5);
    mcu.set_register(2, 4);
    let memory_data = vec![0x12, 0x04];
    mcu.load_program_memory(&memory_data);
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
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0);
    mcu.set_register(2, 255);
    let memory_data = vec![0x12, 0x04];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    mcu.step();
    flags = mcu.get_flags();
    // Set if the absolute value of rr plus carry is larger than absolute value of rd
    assert!(flags.carry);
    assert!(flags.zero);
}

/// Tests simple compare instruction. Rd is less than (rr + C),
/// so carry should be set and zero should be zero
#[test]
fn test_cpc_rd_less() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 128);
    mcu.set_register(2, 128);
    let memory_data = vec![0x12, 0x04];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    mcu.step();
    flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(!flags.zero);
}

/// Tests compare-skip instruction. Rd and rr are not equals
/// No skip should happen
#[test]
fn test_cpse_no_skip() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(6, 5);
    mcu.set_register(20, 255);
    let memory_data = vec![0x46, 0x11, 0x12, 0x0C];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    assert_eq!(mcu.get_program_counter(), 0x0);
    flags = mcu.get_flags();
    mcu.step();
    let new_flags = mcu.get_flags();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(flags, new_flags);
}

/// Tests compare-skip instruction. Rd and rr are equals
/// Next instruction is 1 word, should skip only one
#[test]
fn test_cpse_skip_simple() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(6, 255);
    mcu.set_register(20, 255);
    let memory_data = vec![0x46, 0x11, 0x12, 0x0C];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    assert_eq!(mcu.get_program_counter(), 0x0);
    flags = mcu.get_flags();
    mcu.step();
    let new_flags = mcu.get_flags();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(flags, new_flags);
}

/// Tests compare-skip instruction. Rd and rr are equals
/// Next instruction is 2 word, should skip two words
/// LDS r5, 0x1234 -> 1001 0000 0101 0000 -> 90 50
#[test]
fn test_cpse_skip_double() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(6, 255);
    mcu.set_register(20, 255);
    let memory_data = vec![0x46, 0x11, 0x50, 0x90];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);

    assert_eq!(mcu.get_program_counter(), 0x0);
    flags = mcu.get_flags();
    mcu.step();
    let new_flags = mcu.get_flags();
    assert_eq!(mcu.get_program_counter(), 0x6);
    assert_eq!(flags, new_flags);
}
