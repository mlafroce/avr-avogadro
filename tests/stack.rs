extern crate avr_avogadro;

use avr_avogadro::core::mcu::Mcu;

#[test]
/// Tests stack roll / unroll
///
/// Code generated by compiling
/// `int main() {}`
/// With command
/// `avr-gcc -mmcu=attiny13 -nostartfiles -c`
///
///   0: cf 93         push  r28
///   2: df 93         push  r29
///   4: cd b7         in  r28, 0x3d ; 61
///   6: dd 27         eor r29, r29
///   8: 80 e0         ldi r24, 0x00 ; 0
///   a: 90 e0         ldi r25, 0x00 ; 0
///   c: df 91         pop r29
///   e: cf 91         pop r28
///  10: 08 95         ret
///
/// Since stack is not configured, ret will jump to 93CF + 2 => 93D1
fn test_basic_stack() {
    let mut mcu = Mcu::new();
    let mut memory_data = vec![0xcf, 0x93, 0xdf, 0x93, 0xcd, 0xb7, 0xdd, 0x27, 0x80,
        0xe0, 0x90, 0xe0, 0xdf, 0x91, 0xcf, 0x91, 0x08, 0x95];
    memory_data.resize(1024, 0);
    mcu.load_memory(&memory_data);
    for _ in 0..9 {
        mcu.step()
    }
    assert_eq!(mcu.get_program_counter(), 0x93d1);
}