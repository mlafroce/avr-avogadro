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
