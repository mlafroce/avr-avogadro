extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

/// Tests load immediate
///
/// LDI opcode: 1110 KKKK dddd KKKK
/// ldi r16, 0x1A -> 1110 0001 0000 1010 -> E10A
///
/// Remember AVR is little endian!
#[test]
fn test_ldi() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0x0A, 0xE1, 0x1F, 0xEF, 0x10, 0xE0];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_register(16), 0x1A);
    mcu.step();
    assert_eq!(mcu.get_register(17), 0xFF);
    mcu.step();
    assert_eq!(mcu.get_register(17), 0x00);
}

/// Tests load immediate
///
/// PUSH opcode: 1001 001d dddd 1111
/// push r0   -> 1001 0010 0000 1111 -> 920F
///
/// Remember AVR is little endian!
#[test]
fn test_push() {
    let mut mcu = Mcu::new();
    mcu.set_register(0, 0xDE);
    mcu.set_register(1, 0xAD);
    mcu.set_register(30, 0xBE);
    mcu.set_register(31, 0xEF);
    let mut memory_data = vec![0x0F, 0x92, 0x1F, 0x92, 0xEF, 0x93, 0xFF, 0x93];
    memory_data.resize(1024, 0);
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_stack_pointer(), 0x00);
    for _ in 0..4 {
        mcu.step();
    }
    let mem_max = mcu.get_memory_size() as u16;
    assert_eq!(mcu.get_stack_pointer(), mem_max - 0x04);
    assert_eq!(mcu.get_memory_byte(mem_max - 4), 0xEF);
    assert_eq!(mcu.get_memory_byte(mem_max - 3), 0xBE);
    assert_eq!(mcu.get_memory_byte(mem_max - 2), 0xAD);
    assert_eq!(mcu.get_memory_byte(mem_max - 1), 0xDE);
}