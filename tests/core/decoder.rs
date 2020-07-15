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
    (0x8000, "ld\tr0, Z"), // 10k0 kksd dddd ykkk LDD/STD through Z+k or Y+k
    (0x8008, "ld\tr0, Y"),
    (0x81F0, "ld\tr31, Z"),
    (0x81F8, "ld\tr31, Y"),
    (0x8200, "st\tZ, r0"),
    (0x8208, "st\tY, r0"),
    (0x83F0, "st\tZ, r31"),
    (0x83F8, "st\tY, r31"),
    (0xADF7, "ldd\tr31, Z+63"),
    (0xADFF, "ldd\tr31, Y+63"),
    (0xAFF7, "std\tZ+63, r31"),
    (0xAFFF, "std\tY+63, r31"),
    // lds rd,i/sts will be tested later
    (0x9001, "ld\tr0, Z+"), // 1001 00sd dddd y001: ld/st rd through z+/y+
    (0x9009, "ld\tr0, Y+"),
    (0x91F1, "ld\tr31, Z+"), // warn! undefined
    (0x91F9, "ld\tr31, Y+"),
    (0x9201, "st\tZ+, r0"),
    (0x9209, "st\tY+, r0"),
    (0x93F1, "st\tZ+, r31"), // warn! undefined
    (0x93F9, "st\tY+, r31"),
    (0x9002, "ld\tr0, -Z"), // 1001 00sd dddd y010: ld/st rd through −z/−y
    (0x900A, "ld\tr0, -Y"),
    (0x91f2, "ld\tr31, -Z"), // warn! undefined
    (0x91fa, "ld\tr31, -Y"),
    (0x9202, "st\t-Z, r0"),
    (0x920a, "st\t-Y, r0"),
    (0x93f2, "st\t-Z, r31"), // warn! undefined
    (0x93fa, "st\t-Y, r31"),
    // 1001 000d dddd 01q0 lpm/elpm rd,z
    // 1001 000d dddd 01q1 lpm/elpm rd,z+
    // 1001 001d dddd 0100 xch z,rd
    // 1001 001d dddd 0101 las z,rd
    // 1001 001d dddd 0110 lac z,rd
    // 1001 001d dddd 0111 lat z,rd
    (0x900c, "ld\tr0, X"), // 1001 00sd dddd 1100 ld/st rd through x
    (0x91fc, "ld\tr31, X"),
    (0x920c, "st\tX, r0"),
    (0x93fc, "st\tX, r31"),
    (0x900d, "ld\tr0, X+"), // 1001 00sd dddd 1101 ld/st rd through x+
    (0x91fd, "ld\tr31, X+"),
    (0x920d, "st\tX+, r0"),
    (0x93fd, "st\tX+, r31"),
    (0x900e, "ld\tr0, -X"), // 1001 00sd dddd 1110 ld/st rd through −x
    (0x91fe, "ld\tr31, -X"),
    (0x920e, "st\t-X, r0"),
    (0x93fe, "st\t-X, r31"),
    (0x900f, "pop\tr0"), // 1001 00sd dddd 1111 pop/push rd
    (0x91ff, "pop\tr31"),
    (0x920f, "push\tr0"),
    (0x93ff, "push\tr31"),
    (0x9400, "com\tr0"), // 1001 010d dddd 0000: com rd
    (0x95f0, "com\tr31"), 
    (0x9401, "neg\tr0"), // 1001 010d dddd 0001: neg rd
    (0x95f1, "neg\tr31"),
    (0x9402, "swap\tr0"), // 1001 010d dddd 0010: swap rd
    (0x95f2, "swap\tr31"), 
    (0x9403, "inc\tr0"), // 1001 010d dddd 0011: inc rd
    (0x95f3, "inc\tr31"),
    (0x9404, ".word\t0x9404"),// 1001 010d dddd 0100 (reserved)
    (0x9405, "asr\tr0"), // 1001 010d dddd 0101: asr rd
    (0x95f5, "asr\tr31"),
    (0x9406, "lsr\tr0"), // 1001 010d dddd 0110: lsr rd
    (0x95f6, "lsr\tr31"),
    (0x9407, "ror\tr0"), // 1001 010d dddd 0111: ror rd
    (0x95f7, "ror\tr31"),
    (0x9408, "sec"), // 1001 0100 Bbbb 1000: sex/clx status register clear/set bit
    (0x9418, "sez"),
    (0x9428, "sen"),
    (0x9438, "sev"),
    (0x9448, "ses"),
    (0x9458, "seh"),
    (0x9468, "set"),
    (0x9478, "sei"),
    (0x9488, "clc"), 
    (0x9498, "clz"),
    (0x94a8, "cln"),
    (0x94b8, "clv"),
    (0x94c8, "cls"),
    (0x94d8, "clh"),
    (0x94e8, "clt"),
    (0x94f8, "cli"),
    (0x9508, "ret"), // Zero operand
    (0x9518, "reti"),
    (0x9528, ".word\t0x9528"),
    (0x9588, "sleep"),
    (0x9598, "break"),
    (0x95a8, "wdr"),
    (0x95b8, ".word\t0x95b8"),
    (0x95c8, "lpm"),
    (0x95d8, "elpm"),
    (0x95e8, "spm"),
    (0x95f8, "spm\tz+"),
    //(0x9409, "ijmp"),
    //(0x9419, "eijmp"),
    //(0x9509, "icall"),
    //(0x9519, "eicall"),
    //(0x940a, "dec\tr0"),
    //(0x95fa, "dec\tr31"),
    //(0x940b, "des\t0"),
    //(0x94fb, "des\t15"),
    //// (0x940c,""), JMP/CALL abs22
    //// (0x95fd,""), JMP/CALL abs22
    //// (0x940e,""), JMP/CALL abs22
    //// (0x95ff,""), JMP/CALL abs22
    //(0x9600, "adiw"),
    //(0x96ff, "adiw"),
    //(0x9700, "sbiw"),
    //(0x97ff, "sbiw"),
    //(0x9800, "cbi\t0x00, 0"),
    //(0x98ff, "cbi\t0x1f, 7"),
    //(0x9900, "sbic\t0x00, 0"),
    //(0x99ff, "sbic\t0x1f, 7"),
    //(0x9a00, "sbi\t0x00, 0"),
    //(0x9aff, "sbi\t0x1f, 7"),
    //(0x9b00, "sbis\t0x00, 0"),
    //(0x9bff, "sbis\t0x1f, 7"),
    //(0x9c00, "mul\tr0,r0"),
    //(0x9df0, "mul\tr31,r0"),
    //(0x9e00, "mul\tr0,r16"),
    //(0x9fff, "mul\tr31,r31"),
    (0xb000, "in\tr0, 0x00"),
    (0xb7ff, "in\tr31, 0x3f"),
    (0xb800, "out\t0x00, r0"),
    (0xbfff, "out\t0x3f, r31"),
    (0xc000, "rjmp\t.+0"),
    (0xc7ff, "rjmp\t.+4094"),
    (0xc800, "rjmp\t.-4096"),
    (0xcfff, "rjmp\t.-2"),
    (0xd000, "rcall\t.+0"),
    (0xd7ff, "rcall\t.+4094"),
    (0xd800, "rcall\t.-4096"),
    (0xdfff, "rcall\t.-2"),
    (0xe000, "ldi\tr16, 0x00"),
    (0xe00f, "ldi\tr16, 0x0F"),
    (0xe0f0, "ldi\tr31, 0x00"),
    (0xef00, "ldi\tr16, 0xF0"),
    (0xefff, "ldi\tr31, 0xFF"),
    (0xf000, "brcs\t.+0"), // Conditional branch on status register bit 
    (0xf001, "breq\t.+0"), // 1111 0Bxx xxxx xbbb
    (0xf002, "brmi\t.+0"),
    (0xf003, "brvs\t.+0"),
    (0xf004, "brlt\t.+0"),
    (0xf005, "brhs\t.+0"),
    (0xf006, "brts\t.+0"),
    (0xf007, "brie\t.+0"),
    (0xf008, "brcs\t.+2"),
    (0xf0f0, "brcs\t.+60"),
    (0xf400, "brcc\t.+0"),
    (0xf401, "brne\t.+0"),
    (0xf402, "brpl\t.+0"),
    (0xf403, "brvc\t.+0"),
    (0xf404, "brge\t.+0"),
    (0xf405, "brhc\t.+0"),
    (0xf406, "brtc\t.+0"),
    (0xf407, "brid\t.+0"),
    (0xf408, "brcc\t.+2"),
    (0xf4f0, "brcc\t.+60"),
    //(0xf800, "bld\tr0,0"),
    //(0xf801, "bld\tr0,1"),
    //(0xf808, ".word\t0xf808"),
    //(0xf810, "bld\tr1,0"),
    //(0xfa00, "bst\tr0,0"),
    //(0xfa01, "bst\tr0,1"),
    //(0xfa08, ".word\t0xfa08"),
    //(0xfa10, "bst\tr1,0"),
    //(0xf900, "bld\tr0,0"),
    //(0xf901, "bld\tr0,1"),
    //(0xf908, ".word\t0xf908"),
    //(0xf910, "bld\tr1,0"),
    //(0xfb00, "bst\tr0,0"),
    //(0xfb01, "bst\tr0,1"),
    //(0xfb08, ".word\t0xfb08"),
    //(0xfb10, "bst\tr1,0"),
    (0xffff, ".word\t0xffff")
    ];
    let mut buf = String::new();
    for instruction in instructions_test_set {
        let decoded = Decoder::decode(instruction.0);
        write!(buf, "{}", decoded).unwrap();
        assert_eq!(buf, instruction.1, "0x{:x} expected to be {}", instruction.0, instruction.1);
        buf.clear();
    }
}
