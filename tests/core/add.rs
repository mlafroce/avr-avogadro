extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

#[test]
/// Tests simple add instruction
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
/// Tests simple add instruction which results in zero and carry flag on
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
    assert_eq!(mcu.get_register(1), 0x00);
    let flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(flags.zero);
}

#[test]
/// Tests add with carry instruction
///
/// ADD opcode: 0001 11rd dddd rrrr
/// add r16, r20 -> 0001 1111 0000 0100 -> 1F04
fn test_sum_with_carry() {
    let mut mcu = Mcu::new();
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.set_register(16, 0x04);
    mcu.set_register(20, 0x05);
    // 0x78 + 0x88 = 0x100
    let memory_data = vec![0x04, 0x1F];
    mcu.load_memory(&memory_data);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(16), 0x0A);
    flags = mcu.get_flags();
    assert!(!flags.carry);
}
