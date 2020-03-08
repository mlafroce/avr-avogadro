extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests branch on carry set instruction
///
/// BRCS opcode: 1111 00kk kkkk k000
/// brcs .+20 -> 1111 0000 0101 0000 -> F050
///
/// Remember AVR is little endian!
fn test_branch_carry() {
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

