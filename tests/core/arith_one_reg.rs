extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests One's complement (COM) instruction
///
/// COM opcode: 1001 010d dddd 0000
/// com r24 ->  1001 0101 1000 0000 -> 9580
///
/// Remember AVR is little endian!
///
fn test_com() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x80, 0x95, 0x90, 0x95, 0xA0, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(24, 0);
    mcu.set_register(25, 0xFF);
    mcu.set_register(26, 0x77);
    
    // 0x00 -> 0xFF
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(24), 0xFF);
    assert_eq!(0x15, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x15, flag_as_byte);
    // 0xFF -> 0x00
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(25), 0x00);
    assert_eq!(0x3, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x3, flag_as_byte);
    // 0x77 -> 0x88
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(26), 0x88);
    assert_eq!(0x15, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x15, flag_as_byte);
}


#[test]
/// Tests Two's complement (NEG) instruction
///
/// NEG opcode: 1001 010d dddd 0001
/// neg r24 ->  1001 0101 1000 0001 -> 9581
///
/// Remember AVR is little endian!
///
fn test_neg() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x81, 0x95, 0x91, 0x95, 0xA1, 0x95, 0xB1, 0x95, 0xC1, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(24, 0);
    mcu.set_register(25, 0xFF);
    mcu.set_register(26, 0x77);
    mcu.set_register(27, 0x78);
    mcu.set_register(28, 0x80);
    // 0x00 -> 0x00
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(24), 0);
    assert_eq!(0x22, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x22, flag_as_byte);
    // 0xFF -> 0x01
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(25), 0x01);
    assert_eq!(0x1, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x1, flag_as_byte);
    // 0x77 -> 0x89
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(26), 0x89);
    assert_eq!(0x35, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x35, flag_as_byte);
    // 0x78 -> 0x88
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(27), 0x88);
    assert_eq!(0x35, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x35, flag_as_byte);
    // 0x80 -> 0x80
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(28), 0x80);
    assert_eq!(0x2D, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x2D, flag_as_byte);
}

#[test]
/// Tests Swap Nibbles (SWAP) instruction
///
/// SWAP opcode: 1001 010d dddd 0010
/// swap r16 ->  1001 0101 0000 0010 -> 9502
///
/// Remember AVR is little endian!
///
fn test_swap() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x02, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(16, 0xF2);
    mcu.step();
    assert_eq!(mcu.get_register(16), 0x2F);
}

#[test]
/// Tests increment (INC) instruction
///
/// INC opcode: 1001 010d dddd 0011
/// inc r16 ->  1001 0101 0000 0011 -> 9503
///
/// Remember AVR is little endian!
///
fn test_inc() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x03, 0x95, 0x03, 0x95, 0x03, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(16, 0xFE);
    // 0xFE -> 0xFF
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0xFF);
    assert_eq!(0x14, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x14, flag_as_byte);
    // 0xFF -> 0x00
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0);
    assert_eq!(0x02, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x02, flag_as_byte);
    // Test overflow
    mcu.set_register(16, 0x7F);
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0x80);
    assert_eq!(0xc, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0xc, flag_as_byte);
}

#[test]
/// Tests decrement (DEC) instruction
///
/// INC opcode: 1001 010d dddd 1010
/// dec r16 ->  1001 0101 0000 1010 -> 950A
///
/// Remember AVR is little endian!
///
fn test_dec() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x0A, 0x95, 0x0A, 0x95, 0x0A, 0x95, 0x0A, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(16, 0x02);
    // 0x02 -> 0x01
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0x01);
    assert_eq!(0x0, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x0, flag_as_byte);
    // 0x01 -> 0x00
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0);
    assert_eq!(0x02, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x02, flag_as_byte);
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0xFF);
    assert_eq!(0x14, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x14, flag_as_byte);
    // Test overflow
    mcu.set_register(16, 0x80);
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(16), 0x7F);
    assert_eq!(0x18, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x18, flag_as_byte);
}

#[test]
/// Tests Arithmetic shift right (ASR) instruction
///
/// ASR opcode: 1001 010d dddd 0101
/// asr r18 ->  1001 0101 0010 0101 -> 9525
///
/// Remember AVR is little endian!
///
fn test_asr() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x25, 0x95, 0x25, 0x95];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(18, 0xF2);
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(18), 0xF9);
    assert_eq!(0xC, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0xC, flag_as_byte);
}

#[test]
/// Tests Logic shift right (LSR) instruction
///
/// LSR opcode: 1001 010d dddd 0110
/// lsr r2  ->  1001 0100 0010 0110 -> 9426
///
/// Remember AVR is little endian!
///
fn test_lsr() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x26, 0x94, 0x26, 0x94];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(2, 0xF2);
    mcu.step();
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(2), 0x79);
    assert_eq!(0, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0, flag_as_byte);
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(2), 0x3C);
    assert_eq!(0x19, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x19, flag_as_byte);
}

#[test]
/// Tests Rotate right through carry (ROR) instruction
///
/// ROR opcode: 1001 010d dddd 0111
/// ror r2  ->  1001 0100 0010 0111 -> 9427
///
/// Remember AVR is little endian!
///
fn test_ror() {
    let mut mcu = McuFactory::create("attiny85");
    let program_memory = vec![0x27, 0x94, 0x27, 0x94];
    mcu.load_program_memory(&program_memory);
    mcu.set_register(2, 0xF1);
    mcu.step();
    // carry 0, 0xF1 = 0 1111 0001 -> 1 0111 1000 = carry 1, 0x78
    let mut flag_as_byte : u8 = mcu.get_flags().into();
    assert_eq!(mcu.get_register(2), 0x78);
    assert_eq!(0x19, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0x19, flag_as_byte);
    mcu.step();
    flag_as_byte = mcu.get_flags().into();
    assert_eq!(mcu.get_register(2), 0xBC);
    assert_eq!(0xC, flag_as_byte,
            "Flags assertion failed: {:08b} != {:08b}",
            0xC, flag_as_byte);
}
