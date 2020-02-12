extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

const MEM_MAX: usize = 1024;
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
    memory_data[0x42] = 0x20;
    memory_data[0x43] = 0xD0;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_memory_size() as u16, 1024);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x42);
    assert_eq!(mcu.get_stack_pointer(), mcu.get_memory_size() as u16 - 2);
    assert_eq!(mcu.get_memory_byte(0), 2); // program counter (low) + 2
    assert_eq!(mcu.get_memory_byte(1), 0); // program counter hi
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x84);
    assert_eq!(mcu.get_stack_pointer(), mcu.get_memory_size() as u16 - 4);
    assert_eq!(mcu.get_memory_byte(mcu.get_memory_size() as u16 - 2), 0x44);
    assert_eq!(mcu.get_memory_byte(mcu.get_memory_size() as u16 - 1), 0);
}

/// Tests call to relative negative address
#[test]
fn test_rcall_neg() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; MEM_MAX];
    memory_data[0] = 0x20;
    memory_data[1] = 0xD0;
    memory_data[0x42] = 0xF0;
    memory_data[0x43] = 0xDF;
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    assert_eq!(mcu.get_stack_pointer(), 0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x42);
    assert_eq!(mcu.get_stack_pointer(), MEM_MAX as u16 - 0x02);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x24);
    assert_eq!(mcu.get_stack_pointer(), MEM_MAX as u16 - 0x04);
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
    assert_eq!(mcu.get_stack_pointer(), MEM_MAX as u16 - 0x04);
}

/// Tests ret instruction
/// RET opcode: 1001 0101 0000 1000 = 0x9508
#[test]
fn test_ret() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0; MEM_MAX];
    memory_data[5] = 0xD0; // memory_data[4] = 0 => when pc is 0x6, exec rcall +0
    memory_data[0xE] = 0x08; // memory_data[4] = 0 => rcall +0
    memory_data[0xF] = 0x95; // memory_data[4] = 0 => rcall +0
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    assert_eq!(mcu.get_stack_pointer(), 0);
    mcu.step(); // pc -> 2
    mcu.step(); // pc -> 4
    mcu.step(); // pc -> 6, exec rcall +0
    assert_eq!(mcu.get_program_counter(), 0x6);
    assert_eq!(mcu.get_stack_pointer(), mcu.get_memory_size() as u16 - 2);
    assert_eq!(mcu.get_memory_byte(0), 0x6);
    mcu.step(); // pc -> 8
    mcu.step(); // pc -> A
    mcu.step(); // pc -> C
    mcu.step(); // pc -> E 
    assert_eq!(mcu.get_program_counter(), 0xE);
    mcu.step(); // exec ret -> PC should be 0x6 + 2
    assert_eq!(mcu.get_stack_pointer(), 0);
    assert_eq!(mcu.get_program_counter(), 0x8);
    mcu.step(); // pc -> E     
    assert_eq!(mcu.get_program_counter(), 0xA);
}
