extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

/// Tests load from I/O Location
/// IO Address have a 0x20 byte offset
///
/// IN opcode:      1011 0AAd dddd AAAA
/// in r25, 0x16 -> 1011 0011 1001 0101 -> B395
///
/// Remember AVR is little endian!
#[test]
fn test_in() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0; 1024];
    memory_data[0] = 0x95;
    memory_data[1] = 0xB3;
    memory_data[0x36] = 0x66;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(25), 0x66);
}

/// Tests store into I/O Location
/// IO Address have a 0x20 byte offset
///
/// OUT opcode:      1011 1AAr rrrr AAAA
/// out r25, 0x16 -> 1011 1011 1001 0101 -> BB95
///
/// Remember AVR is little endian!
#[test]
fn test_in() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0; 1024];
    memory_data[0] = 0x95;
    memory_data[1] = 0xBB;
    mcu.set_register(25, 0x42);
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_memory_byte(0x36), 0x42);
}
