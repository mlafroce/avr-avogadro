extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

const mem_max: usize = 1024;
/// Tests call to relative address
/// Address offset can be negative or positive
///
/// RCALL opcode: 1101 KKKK KKKK KKKK
/// rcall 0x20 -> 1110 0000 0010 0000 -> D020
///
/// Remember AVR is little endian!
#[test]
fn test_rcall() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[0] = 0x20;
    memory_data[1] = 0xD0;
    memory_data[0x22] = 0x20;
    memory_data[0x23] = 0xD0;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x22);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x44);
}

/// Tests call to relative negative address
#[test]
fn test_rcall_neg() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; mem_max];
    memory_data[0] = 0x20;
    memory_data[1] = 0xD0;
    memory_data[0x22] = 0xF0;
    memory_data[0x23] = 0xDF;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    assert_eq!(mcu.get_stack_pointer(), 0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x22);
    assert_eq!(mcu.get_stack_pointer(), mem_max as u16 - 0x02);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x14);
    assert_eq!(mcu.get_stack_pointer(), mem_max as u16 - 0x04);
}

/// Typical instruction to make some stack space
#[test]
fn test_rcall_zero() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; 1024];
    memory_data[1] = 0xD0;
    memory_data[3] = 0xD0;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x4);
    assert_eq!(mcu.get_stack_pointer(), mem_max as u16 - 0x04);
}