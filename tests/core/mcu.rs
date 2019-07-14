extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;
use avr_avogadro::core::memory_bank::MemoryBank;

#[test]
/// Tests símple Nop instruction
/// Program counter should advance
fn test_step() {
    let mut mcu = Mcu::new();
    let memory_data = vec![0, 0];
    mcu.load_memory(&memory_data);
    assert_eq!(mcu.get_program_counter(), 0x0);
    mcu.step();
    assert_eq!(mcu.get_program_counter(), 0x2);
}
