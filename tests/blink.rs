extern crate avr_avogadro;

use avr_avogadro::core::mcu_factory::McuFactory;
use std::path::PathBuf;

const PORT_B: u16 = 0x38;

#[test]
/// Tests the most basic "Blink" application, widely known for making a
/// LED blink every second
///
/// Binary generated by compiling `03-blink.c` in the "examples" folder
///
/// Compiled with
/// avr-gcc -O2 -Wextra -Wall -mmcu=attiny85 03-blink.c -o 03-blink.o
/// avr-objcopy -j .text -j .data -O binary 03-blink.o 03-blink.bin
///
/// ## The loop
///
/// 3c:   e7 ea           ldi     r30, 0xA7       ; 167
/// 3e:   f1 e6           ldi     r31, 0x61       ; 97
/// 40:   31 97           sbiw    r30, 0x01       ; 1
/// 42:   f1 f7           brne    .-4             ;  0x40
///
/// After that we toggle PORTB value
///
///  44:   00 c0           rjmp    .+0             ; 0x46
///  46:   00 00           nop
///  48:   f6 cf           rjmp    .-20            ; 0x36
///  36:   88 b3           in      r24, 0x18       ; 24
///  38:   89 27           eor     r24, r25
///  3a:   88 bb           out     0x18, r24       ; 24
///
/// Register Z (30:31) is initiated with value 0xA7, 0x61, which is 24999 in little endian
/// At line 0x40, 1 is substracted from reg Z, then zero flag is checked. If it's not zero,
/// substract again
/// `sbiw` instructions costs 2 mcu cycles, while branch takes 2 cycles when it's true and 1 otherwise
/// 24999 * (2 + 2)-> 99998 cycles, almost 100ms in a 1MHz mcu

fn test_blink() {
    let mut mcu = McuFactory::create("attiny85");
    let mut pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pathbuf.push("tests");
    pathbuf.push("blink.bin");
    let path = pathbuf.into_os_string().into_string().unwrap();
    mcu.load_from_file(&path, true).unwrap();
    mcu.load_from_file(&path, true).unwrap();
    assert_eq!(mcu.get_data_byte(PORT_B), 0);
    mcu.step_n(16);
    assert_eq!(mcu.get_program_counter(), 0x40);
    assert_eq!(mcu.get_data_byte(PORT_B), 1);
    mcu.step_n(50006);
    assert_eq!(mcu.get_data_byte(PORT_B), 0);
    mcu.step_n(50006);
    assert_eq!(mcu.get_data_byte(PORT_B), 1);
    mcu.step_n(50006);
    assert_eq!(mcu.get_data_byte(PORT_B), 0);
    mcu.step_n(50006);
    assert_eq!(mcu.get_data_byte(PORT_B), 1);
    mcu.step_n(50006);
    assert_eq!(mcu.get_data_byte(PORT_B), 0);
}
