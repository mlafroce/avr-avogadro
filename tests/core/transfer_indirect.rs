extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

/// Tests load indirect
///
/// LDD opcode:     10q0 qq0d dddd yqqq
/// if y == 1, base reg is Y, else Z
/// ldd Y+2, r24 -> 1000 0001 1000 1010 -> 818A
///
/// Remember AVR is little endian!
#[test]
fn test_ldd() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[0] = 0x8A;
    memory_data[1] = 0x81;
    memory_data[0x10] = 42;
    mcu.set_register(28, 0xE);
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step(); // Y + 2 = 40 + 2
    assert_eq!(mcu.get_register(24), 42); 
}

/// Tests load indirect with Z
///
/// LDD opcode:      10q0 qq0d dddd yqqq
/// ldd Z+63, r24 -> 1010 1101 1000 0111 -> AD87
#[test]
fn test_ldd_z() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[0] = 0x87;
    memory_data[1] = 0xAD;
    memory_data[0x300] = 42;  // 0x2C1 + 0x3F = 0x300 
    mcu.set_register(30, 0xC1);
    mcu.set_register(31, 0x2);
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step(); // Y + 2 = 40 + 2
    assert_eq!(mcu.get_register(24), 42); 
}

/// Tests store indirect
///
/// STD opcode:     10q0 qq1d dddd yqqq
/// if y == 1, base reg is Y, else Z
/// std Y+42, r0 -> 1010 0110 0000 1010 -> A60A
#[test]
fn test_std() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[0] = 0x0A;
    memory_data[1] = 0xA6;
    mcu.load_memory(&memory_data);
    mcu.set_register(28, 0x16); // 0x2A + 0x16 = 0x40
    mcu.set_register(0, 42);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_memory_byte(0x40), 42); 
}

/// Tests store indirect with Z
///
/// STD opcode:      10q0 qq1d dddd yqqq
/// if y == 1, base reg is Y, else Z
/// std Y+21, r15 -> 1000 1010 1111 0101 -> 8AF5
#[test]
fn test_std_z() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[0] = 0xF5;
    memory_data[1] = 0x8A;
    mcu.load_memory(&memory_data);
    mcu.set_register(30, 0xEA);
    mcu.set_register(31, 0x03); // 0x3EA + 0x15 = 3FF
    mcu.set_register(15, 42);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_memory_byte(0x3FF), 42); 
}
