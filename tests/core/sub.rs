extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

/// Tests simple substract instruction
///
/// SUB opcode: 0001 10rd dddd rrrr
/// sub r1, r2 -> 0001 1000 0001 0010 -> 1812
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

/// Tests simple substract instruction, 1 minus 0
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

/// Tests simple substract instruction, 3 minus -4
#[test]
fn test_subi_pos_minus_neg() {
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

/// Tests simple substract instruction, 0 minus 8
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

/// ADD General test cases based on RISC-V test set
/// SUB opcode: 0001 10rd dddd rrrr
/// rr <- rd - rr
#[test]
fn test_sub_general() {
    let mut mcu = Mcu::new();
    // sub r1, r2 -> 0001 1000 0001 0010 -> 1812
    let memory_data = vec![0x12, 0x18];
    mcu.load_memory(&memory_data);

    // rd, rr, result, flags (ithsvnzc)
    let test_set = vec![
    (0x00, 0x00, 0x00, 0b00000010),
    (0x01, 0x01, 0x00, 0b00000010),
    (0x01, 0x00, 0x01, 0b00000000),
    (0x00, 0x01, 0xFF, 0b00110101),
    (0x10, 0x01, 0x0F, 0b00100000),
    (0x80, 0x80, 0x00, 0b00000010)
    ];
    for (i, test_case) in test_set.iter().enumerate() {
        mcu.set_program_counter(0);
        mcu.set_register(1, test_case.0);
        mcu.set_register(2, test_case.1);
        mcu.step();

        let result = mcu.get_register(1);
        let flag_as_byte : u8 = mcu.get_flags().into();
        assert_eq!(result, test_case.2,
            "Test case {} - Sub assertion failed: {:x} != {:x}", i, result, test_case.2);
        assert_eq!(flag_as_byte, test_case.3,
            "Test case {} - Flags assertion failed: {:08b} != {:08b}",
             i, flag_as_byte, test_case.3);
    }
}
