extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests add immediate to word (ADIW) instruction
///
/// ADIW opcode:     1001 0110 KKdd KKKK
/// adiw r25:24,1 -> 1001 0110 0000 0001 -> 9601
///
/// Remember AVR is little endian!
///
fn test_adiw_one() {
    let mut mcu = McuFactory::create("attiny85");
    // adiw r25:24,1 -> 1001 0110 0000 0001 -> 9601
    // adiw r27:26,1 -> 1001 0110 0001 0001 -> 9611
    let memory_data = vec![0x01, 0x96, 0x11, 0x96, 0x21, 0x96, 0x31, 0x96];
    mcu.load_program_memory(&memory_data);
    mcu.set_program_counter(0);
    mcu.set_register(24, 0);
    mcu.set_register(25, 0);
    mcu.set_register(26, 0xFF);
    mcu.set_register(27, 0);
    mcu.set_register(28, 0xFF);
    mcu.set_register(29, 0x7F);
    mcu.set_register(30, 0xFF);
    mcu.set_register(31, 0xFF);

    // 0x0000 + 1
    mcu.step();
    assert_eq!(mcu.get_register(24), 1);
    assert_eq!(mcu.get_register(25), 0);
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(0, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0, flag_as_byte);

    // 0x00FF + 1
    mcu.step();
    assert_eq!(mcu.get_register(26), 0);
    assert_eq!(mcu.get_register(27), 1);
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(0, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0, flag_as_byte);

    // 0x7FFF + 1
    mcu.step();
    assert_eq!(mcu.get_register(28), 0);
    assert_eq!(mcu.get_register(29), 0x80);
    flag_as_byte = mcu.get_flags().into();
    // flags (ithsvnzc)
    assert_eq!(0x0C, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x14, flag_as_byte);

    // 0xFFFF + 1
    mcu.step();
    assert_eq!(mcu.get_register(30), 0);
    assert_eq!(mcu.get_register(31), 0);
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(0x3, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x3, flag_as_byte);
}

#[test]
/// Tests substract immediate from word (SBIW) instruction
///
/// SBIW opcode:     1001 0111 KKdd KKKK
/// sbiw r25:24,1 -> 1001 0111 0000 0001 -> 9601
///
/// Remember AVR is little endian!
///
fn test_sbiw_one() {
    let mut mcu = McuFactory::create("attiny85");
    // sbiw r25:24,1 -> 1001 0111 0000 0001 -> 9701
    // sbiw r27:26,1 -> 1001 0111 0001 0001 -> 9711
    let memory_data = vec![0x01, 0x97, 0x11, 0x97, 0x21, 0x97, 0x31, 0x97];
    mcu.load_program_memory(&memory_data);
    mcu.set_program_counter(0);
    mcu.set_register(24, 1);
    mcu.set_register(25, 0);
    mcu.set_register(26, 0x0);
    mcu.set_register(27, 0x1);
    mcu.set_register(28, 0x0);
    mcu.set_register(29, 0x80);
    mcu.set_register(30, 0x0);
    mcu.set_register(31, 0x0);

    // 0x0000 + 1
    mcu.step();
    assert_eq!(mcu.get_register(24), 0);
    assert_eq!(mcu.get_register(25), 0);
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(0x2, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0, flag_as_byte);

    // 0x00FF + 1
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(26), 0xFF);
    assert_eq!(mcu.get_register(27), 0);
    assert_eq!(0, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0, flag_as_byte);

    // 0x7FFF + 1
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(28), 0xFF);
    assert_eq!(mcu.get_register(29), 0x7F);

    // flags (ithsvnzc)
    assert_eq!(0x18, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x14, flag_as_byte);

    // 0xFFFF + 1
    mcu.step();
    flag_as_byte = mcu.get_flags().into();

    assert_eq!(mcu.get_register(30), 0xFF);
    assert_eq!(mcu.get_register(31), 0xFF);
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(0b00010101, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0b00010101, flag_as_byte);
}
