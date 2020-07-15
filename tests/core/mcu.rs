extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;

#[test]
/// Tests simple Nop instruction
/// Program counter should advance
fn test_step() {
    let mut mcu = McuFactory::create("attiny85");
    let memory_data = vec![0, 0];
    mcu.load_program_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
}
