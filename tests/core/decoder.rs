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
    (0x9001,"ld\tr0, Z+"), // 1001 00sd dddd y001: ld/st rd through z+/y+
    (0x9009,"ld\tr0, Y+"),
    (0x91F1,"ld\tr31, Z+"), // warn! undefined
    (0x91F9,"ld\tr31, Y+"),
    (0x9201,"st\tZ+, r0"),
    (0x9209,"st\tY+, r0"),
    (0x93F1,"st\tZ+, r31"), // warn! undefined
    (0x93F9,"st\tY+, r31"),
    (0x9002,"ld\tr0, -Z"), // 1001 00sd dddd y010: ld/st rd through −z/−y
    (0x900A,"ld\tr0, -Y"),
    (0x91f2,"ld\tr31, -Z"), // warn! undefined
    (0x91fa,"ld\tr31, -Y"),
    (0x9202,"st\t-Z, r0"),
    (0x920a,"st\t-Y, r0"),
    (0x93f2,"st\t-Z, r31"), // warn! undefined
    (0x93fa,"st\t-Y, r31"),
    // 1001 000d dddd 01q0 lpm/elpm rd,z
    // 1001 000d dddd 01q1 lpm/elpm rd,z+
    // 1001 001d dddd 0100 xch z,rd
    // 1001 001d dddd 0101 las z,rd
    // 1001 001d dddd 0110 lac z,rd
    // 1001 001d dddd 0111 lat z,rd
    (0x900c,"ld\tr0, X"), // 1001 00sd dddd 1100 ld/st rd through x
    (0x91fc,"ld\tr31, X"),
    (0x920c,"st\tX, r0"),
    (0x93fc,"st\tX, r31"),
    (0x900d,"ld\tr0, X+"), // 1001 00sd dddd 1101 ld/st rd through x+
    (0x91fd,"ld\tr31, X+"),
    (0x920d,"st\tX+, r0"),
    (0x93fd,"st\tX+, r31"),
    (0x900e,"ld\tr0, -X"), // 1001 00sd dddd 1110 ld/st rd through −x
    (0x91fe,"ld\tr31, -X"),
    (0x920e,"st\t-X, r0"),
    (0x93fe,"st\t-X, r31"),
    (0x900f,"pop\tr0"), // 1001 00sd dddd 1111 pop/push rd
    (0x91ff,"pop\tr31"),
    (0x920f,"push\tr0"),
    (0x93ff,"push\tr31"),
    (0x9400,"com\tr0"), // 1001 010d dddd 0000: com rd
    (0x95f0,"com\tr31"), 
    (0x9401,"neg\tr0"), // 1001 010d dddd 0001: neg rd
    (0x95f1,"neg\tr31"),
    (0x9402,"swap\tr0"), // 1001 010d dddd 0010: swap rd
    (0x95f2,"swap\tr31"), 
    (0x9403,"inc\tr0"), // 1001 010d dddd 0011: inc rd
    (0x95f3,"inc\tr31"),
    (0x9404,"????"),// 1001 010d dddd 0100 (reserved)
    (0x9405,"asr\tr0"), // 1001 010d dddd 0101: asr rd
    (0x95f5,"asr\tr31"),
    (0x9406,"lsr\tr0"), // 1001 010d dddd 0110: lsr rd
    (0x95f6,"lsr\tr31"),
    (0x9407,"ror\tr0"), // 1001 010d dddd 0111: ror rd
    (0x95f7,"ror\tr31"),
    // (0x9408,"sec"), // 1001 0100 bbbb 1000: sex/clx status register clear/set bit
    // (0x9418,"sec"),
    // (0x9428,"sec"),
    // (0x9438,"sec"),
    // (0x9448,"sec"),
    // (0x9458,"sec"),
    // (0x9468,"sec"),
    // (0x9478,"sec"),
    // (0x9488,"sec"),
    ];
    let mut buf = String::new();
    for instruction in instructions_test_set {
        let decoded = Decoder::decode(instruction.0);
        write!(buf, "{}", decoded).unwrap();
        assert_eq!(buf, instruction.1);
        buf.clear();
    }
}
