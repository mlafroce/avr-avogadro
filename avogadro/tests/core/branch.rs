extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests branch on carry set instruction
///
/// BRCS opcode: 1111 00kk kkkk k000
/// brcs .+20 -> 1111 0000 0101 0000 -> F050
///
/// Remember AVR is little endian!
fn test_branch_carry_set() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0x50, 0xF0, 0x50, 0xF0];
    mcu.load_program_memory(&memory_data);
    // Should not branch
    mcu.step();

    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x18); // 0x2 + 0x14 + 0x2
}

#[test]
/// Tests branch on carry clear instruction
///
/// BRCS opcode: 1111 01kk kkkk k000
/// brcs .+20 -> 1111 0100 0101 0000 -> F450
///
/// Remember AVR is little endian!
fn test_branch_carry_clear() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0x50, 0xF4, 0x50, 0xF4];
    mcu.load_program_memory(&memory_data);
    let mut flags = mcu.get_flags();
    flags.carry = true;
    mcu.set_flags(flags);
    // Should not branch
    mcu.step();

    flags.carry = false;
    mcu.set_flags(flags);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x18); // 0x2 + 0x14 + 0x2
}

#[test]
/// Tests branch on bit set instruction
///
/// BRBS opcode: 1111 00kk kkkk kbbb
/// brcs .+4 ->  1111 0000 0001 0bbb -> F01K
///
/// with K in [0..7]
///
/// Remember AVR is little endian!
fn test_branch_bit_set() {
    let mut mcu = McuFactory::create("attiny85");
    let mut memory_data = vec![0 as u8; mcu.get_program_size()];
    for i in 0..8 {
        memory_data[i * 2] = 0x10 + i as u8;
        memory_data[i * 2 + 1] = 0xF0;
    }
    // Branches above should fail
    for i in 0..8 {
        memory_data[i * 6 + 16] = 0x10 + i as u8;
        memory_data[i * 6 + 17] = 0xF0;
    }
    mcu.load_program_memory(memory_data.as_ref());
    let mut flags = mcu.get_flags();
    flags.carry = false;
    flags.zero = false;
    flags.neg = false;
    flags.over = false;
    flags.sign = false;
    flags.half = false;
    flags.trans = false;
    flags.int = false;
    mcu.set_flags(flags);
    let mut last_pc = 0;
    let mut current_pc = 0;
    for _ in 0..8 {
        mcu.step();
        current_pc = mcu.get_program_counter();
        assert_eq!(current_pc, last_pc + 2);
        last_pc = current_pc;
    }
    flags.carry = true;
    flags.zero = true;
    flags.neg = true;
    flags.over = true;
    flags.sign = true;
    flags.half = true;
    flags.trans = true;
    flags.int = true;
    mcu.set_flags(flags);
    for _ in 0..8 {
        mcu.step();
        current_pc = mcu.get_program_counter();
        assert_eq!(current_pc, last_pc + 6);
        last_pc = current_pc;
    }
}
