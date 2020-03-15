extern crate avr_avogadro;

use avr_avogadro::core::decoder::Decoder;
use std::fmt::Write;

#[test]
/// Tests name display for decoded instructions
///
/// Remember AVR is little endian! Instructions are in big endian for ease of reading
fn test_instruction_display() {
    let instructions_test_set: Vec<(u16, &str)> = vec![
    (0x0000, "nop"), // 0000 0000 0000 0000: nop
    (0x0101, "movw\tr0, r2"), // 0000 0001 DDDD RRRR: movw rd, rr (move register pair)
    (0x0200, "muls\tr16, r16"), // 0000 0010 dddd rrrr: muls rd,rr
    (0x02FF, "muls\tr31, r31"), 
    (0x0300, "mulsu\tr16, r16"), // 0000 0011 0ddd 0rrr: mulsu rd,rr
    (0x0377, "mulsu\tr23, r23"),
    (0x0308, "fmul\tr16, r16"), // 0000 0011 0ddd 1rrr: fmul rd,rr
    (0x037F, "fmul\tr23, r23"),
    (0x0380, "fmuls\tr16, r16"), // 0000 0011 1ddd urrr: fmuls(u) rd,rr
    (0x0388, "fmulsu\tr16, r16"),
    (0x03F7, "fmuls\tr23, r23"),
    (0x03FF, "fmulsu\tr23, r23"),
    (0x0400, "cpc\tr0, r0"), // 000c 01rd dddd rrrr: cpc/cp rd,rr
    (0x07FF, "cpc\tr31, r31"),
    (0x1400, "cp\tr0, r0"),
    (0x17FF, "cp\tr31, r31"),
    (0x0800, "sbc\tr0, r0"), // 000c 10rd dddd rrrr: sbc/sub rd,rr
    (0x0BFF, "sbc\tr31, r31"),
    (0x1800, "sub\tr0, r0"),
    (0x1BFF, "sub\tr31, r31"),
    (0x0C00, "add\tr0, r0"), // 000c 11rd dddd rrrr: add/adc rd,rr (lsl/rol rdwhen rd=rr)
    (0x0FFF, "add\tr31, r31"),
    (0x1C00, "adc\tr0, r0"),
    (0x1FFF, "adc\tr31, r31"),
    (0x1000, "cpse\tr0, r0"), // 0001 00rd dddd rrrr: cpse rd,rr
    (0x13FF, "cpse\tr31, r31"),
    (0x2000, "and\tr0, r0"), // 0010 00rd dddd rrrr: and rd,rr
    (0x23FF, "and\tr31, r31"),
    (0x2400, "eor\tr0, r0"), // 0010 01rd dddd rrrr: eor rd,rr
    (0x17FF, "cp\tr31, r31"),
    (0x2800, "or\tr0, r0"), // 0010 10rd dddd rrrr: or rd,rr
    (0x2BFF, "or\tr31, r31"),
    (0x2C00, "mov\tr0, r0"), // 0010 11rd dddd rrrr: mov rd,rr
    (0x2FFF, "mov\tr31, r31"),
    (0x3000, "cpi\tr16, 0x00"), // 0011 kkkk dddd kkkk: cpi rd,k
    (0x303F, "cpi\tr19, 0x0F"),
    (0x3FFF, "cpi\tr31, 0xFF"),
    (0x4000, "sbci\tr16, 0x00"), // 010c kkkk dddd kkkk: sbci/subi rd,k
    (0x403F, "sbci\tr19, 0x0F"),
    (0x4FFF, "sbci\tr31, 0xFF"),
    (0x5000, "subi\tr16, 0x00"),
    (0x503F, "subi\tr19, 0x0F"),
    (0x5FFF, "subi\tr31, 0xFF"),
    (0x6000, "ori\tr16, 0x00"), // 0110 kkkk dddd kkkk: ori rd,k (sbr rd,k)
    (0x603F, "ori\tr19, 0x0F"),
    (0x6FFF, "ori\tr31, 0xFF"),
    (0x7000, "andi\tr16, 0x00"), // 0111 kkkk dddd kkkk: andi rd,k (cbr rd,k)
    (0x703F, "andi\tr19, 0x0F"),
    (0x7FFF, "andi\tr31, 0xFF"),
    ];
    let mut buf = String::new();
    for instruction in instructions_test_set {
        println!("Decoding: {:x}", instruction.0);
        let decoded = Decoder::decode(instruction.0);
        write!(buf, "{}", decoded).unwrap();
        println!("Decoded: {}", buf);
        assert_eq!(buf, instruction.1);
        buf.clear();
    }
}