extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

/// Tests load from I/O Location
/// IO Address have a 0x20 byte offset
///
/// IN opcode:      1011 0AAd dddd AAAA
/// in r25, 0x15 -> 1011 0011 1001 0101 -> B395
///
/// Remember AVR is little endian!
#[test]
fn test_in() {
    let mut mcu = McuFactory::create("attiny85");
    let mut program_memory = vec![0; mcu.get_program_size()];
    program_memory[0] = 0x95;
    program_memory[1] = 0xB3;
    let mut data_memory = vec![0; mcu.get_data_size()];
    data_memory[0x35] = 0x66;
    mcu.load_program_memory(&program_memory);
    mcu.load_data_memory(&data_memory);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(25), 0x66);
}

/// Tests store into I/O Location
/// IO Address have a 0x20 byte offset
///
/// OUT opcode:      1011 1AAr rrrr AAAA
/// out r25, 0x15 -> 1011 1011 1001 0101 -> BB95
///
/// Remember AVR is little endian!
#[test]
fn test_out() {
    let mut mcu = McuFactory::create("attiny85");
    let mut memory_program = vec![0; mcu.get_program_size()];
    memory_program[0] = 0x95;
    memory_program[1] = 0xBB;
    mcu.set_register(25, 0x42);
    mcu.load_program_memory(&memory_program);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_data_byte(0x35), 0x42);
}
