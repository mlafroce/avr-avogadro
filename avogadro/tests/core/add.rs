extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests simple add instruction
///
/// ADD opcode: 0000 11rd dddd rrrr
/// add r1, r2 -> 0000 1100 0001 0002 -> 0C12
///
/// Remember AVR is little endian!
fn test_sum() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 4);
    mcu.set_register(2, 5);
    // 0x04 + 0x05 = 0x9
    let memory_data = vec![0x12, 0x0C];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x09);
}

#[test]
/// Tests simple add instruction which results in zero and carry flag on
fn test_add_flags() {
    let mut mcu = McuFactory::create("attiny85");
    mcu.set_register(1, 0x78);
    mcu.set_register(2, 0x88);
    // 0x78 + 0x88 = 0x100
    let memory_data = vec![0x12, 0x0C];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(1), 0x00);
    let flags = mcu.get_flags();
    assert!(flags.carry);
    assert!(flags.zero);
    assert!(!flags.neg);
    assert!(!flags.over);
    assert!(!flags.sign);
    assert!(flags.half);
}

#[test]
/// ADD General test cases based on RISC-V test set
///
fn test_sum_general() {
    let mut mcu = McuFactory::create("attiny85");
    // add r1, r2 -> 0000 1100 0001 0010 -> 0C12
    let memory_data = vec![0x12, 0x0C];
    mcu.load_program_memory(&memory_data);

    // rd, rr, result, flags (ithsvnzc)
    let test_set = vec![
        (0x00, 0x00, 0x00, 0b00000010),
        (0x01, 0x01, 0x02, 0b00000000),
        (0x03, 0x07, 0x0a, 0b00000000),
        (0xf8, 0x00, 0xf8, 0b00010100),
        (0xff, 0x01, 0x00, 0b00100011),
        (0x01, 0x7f, 0x80, 0b00101100),
        (0x40, 0x40, 0x80, 0b00001100),
        (0xA0, 0xA0, 0x40, 0b00011001),
        (0xC0, 0xC0, 0x80, 0b00010101),
    ];
    for (i, test_case) in test_set.iter().enumerate() {
        mcu.set_program_counter(0);
        mcu.set_register(1, test_case.0);
        mcu.set_register(2, test_case.1);
        mcu.step();

        let result = mcu.get_register(1);
        let flag_as_byte: u8 = mcu.get_flags().into();
        assert_eq!(
            result, test_case.2,
            "Test case {} - Add assertion failed: {:x} != {:x}",
            i, result, test_case.2
        );
        assert_eq!(
            flag_as_byte, test_case.3,
            "Test case {} - Flags assertion failed: {:08b} != {:08b}",
            i, flag_as_byte, test_case.3
        );
    }
}

#[test]
/// Tests add with carry instruction
///
/// ADC opcode: 0001 11rd dddd rrrr
/// adc r16, r20 -> 0001 1111 0000 0100 -> 1F04
fn test_adc_with_carry() {
    let mut mcu = McuFactory::create("attiny85");
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.set_register(16, 0x04);
    mcu.set_register(20, 0x05);
    let memory_data = vec![0x04, 0x1F];
    mcu.load_program_memory(&memory_data);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
    assert_eq!(mcu.get_register(16), 0x0A);
    flags = mcu.get_flags();
    assert!(!flags.carry);
    assert!(!flags.zero);
    assert!(!flags.neg);
    assert!(!flags.over);
    assert!(!flags.sign);
    assert!(!flags.half);
}

#[test]
/// ADC General test cases based on RISC-V test set
///
fn test_adc_with_carry_general() {
    let mut mcu = McuFactory::create("attiny85");
    // adc r1, r2 -> 0001 1100 0001 0010 -> 1C12
    let memory_data = vec![0x12, 0x1C];
    mcu.load_program_memory(&memory_data);

    // rd, rr, result, flags (ithsvnzc)
    let add_test_set = vec![
        (0x00, 0x00, 0x00, 0b00000010),
        (0x01, 0x01, 0x02, 0b00000000),
        (0x03, 0x07, 0x0a, 0b00000000),
        (0xf8, 0x00, 0xf8, 0b00010100),
        (0xff, 0x01, 0x00, 0b00100011),
        (0x01, 0x7f, 0x80, 0b00101100),
        (0x40, 0x40, 0x80, 0b00001100),
        (0xA0, 0xA0, 0x40, 0b00011001),
        (0xC0, 0xC0, 0x80, 0b00010101),
    ];
    for (i, test_case) in add_test_set.iter().enumerate() {
        let mut flags = mcu.get_flags();
        flags.carry = false;
        mcu.set_flags(flags);
        mcu.set_program_counter(0);
        mcu.set_register(1, test_case.0);
        mcu.set_register(2, test_case.1);
        mcu.step();

        let result = mcu.get_register(1);
        let flag_as_byte: u8 = mcu.get_flags().into();
        assert_eq!(
            result, test_case.2,
            "Test case {} - Add assertion failed: {:x} != {:x}",
            i, result, test_case.2
        );
        assert_eq!(
            flag_as_byte, test_case.3,
            "Test case {} - Flags assertion failed: {:08b} != {:08b}",
            i, flag_as_byte, test_case.3
        );
    }

    // rd, rr, result, flags (ithsvnzc)
    let adc_test_set = vec![
        (0x00, 0x00, 0x01, 0b00000000),
        (0x01, 0x01, 0x03, 0b00000000),
        (0x03, 0x07, 0x0b, 0b00000000),
        (0xf8, 0x00, 0xf9, 0b00010100),
        (0xff, 0x00, 0x00, 0b00100011),
        (0xff, 0x01, 0x01, 0b00100001),
        (0x01, 0x7e, 0x80, 0b00101100),
        (0x01, 0x7f, 0x81, 0b00101100),
        (0x40, 0x40, 0x81, 0b00001100),
        (0xA0, 0xA0, 0x41, 0b00011001),
        (0xC0, 0xC0, 0x81, 0b00010101),
    ];
    for (i, test_case) in adc_test_set.iter().enumerate() {
        let mut flags = mcu.get_flags();
        flags.carry = true;
        mcu.set_flags(flags);
        mcu.set_program_counter(0);
        mcu.set_register(1, test_case.0);
        mcu.set_register(2, test_case.1);
        mcu.step();

        let result = mcu.get_register(1);
        let flag_as_byte: u8 = mcu.get_flags().into();
        assert_eq!(
            result, test_case.2,
            "Test case {} - Add assertion failed: {:x} != {:x}",
            i, result, test_case.2
        );
        assert_eq!(
            flag_as_byte, test_case.3,
            "Test case {} - Flags assertion failed: {:08b} != {:08b}",
            i, flag_as_byte, test_case.3
        );
    }
}
