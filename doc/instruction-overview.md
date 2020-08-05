# AVR Instruction set overview

Summary of AVR instruction set, taken from wikipedia.

* Misc

0000 0000 0000 0000 nop
0000 0001 DDDD RRRR movw rd, rr (move register pair)

* Multiplication

0000 0010 dddd rrrr muls rd,rr
0000 0011 0ddd 0rrr mulsu rd,rr
0000 0011 0ddd 1rrr fmul rd,rr
0000 0011 1ddd urrr fmuls(u) rd,rr

* 2 operand instructions

000c 01rd dddd rrrr cpc/cp rd,rr
000c 10rd dddd rrrr sbc/sub rd,rr
000c 11rd dddd rrrr add/adc rd,rr (lsl/rol rdwhen rd=rr)
0001 00rd dddd rrrr cpse rd,rr
0010 00rd dddd rrrr and rd,rr
0010 01rd dddd rrrr eor rd,rr
0010 10rd dddd rrrr or rd,rr
0010 11rd dddd rrrr mov rd,rr

* Register immediate operations

0011 kkkk dddd kkkk cpi rd,k
010c kkkk dddd kkkk sbci/subi rd,k
0110 kkkk dddd kkkk ori rd,k (sbr rd,k)
0111 kkkk dddd kkkk andi rd,k (cbr rd,k)

* Load store operations

10k0 kksd dddd ykkk LDD/STD through Z+k or Y+k
1001 00sd dddd 0000 iiii iiii iiii iiii lds rd,i/sts i,rd (i: 16-bit immediate sram address)
1001 00sd dddd y001 ld/st rd through z+/y+
1001 00sd dddd y010 ld/st rd through −z/−y
1001 000d dddd 01q0 lpm/elpm rd,z
1001 000d dddd 01q1 lpm/elpm rd,z+
1001 001d dddd 0100 xch z,rd
1001 001d dddd 0101 las z,rd
1001 001d dddd 0110 lac z,rd
1001 001d dddd 0111 lat z,rd
1001 00sd dddd 1100 ld/st rd through x
1001 00sd dddd 1101 ld/st rd through x+
1001 00sd dddd 1110 ld/st rd through −x
1001 00sd dddd 1111 pop/push rd

* one-operand instructions

1001 010d dddd 0000 com rd
1001 010d dddd 0001 neg rd
1001 010d dddd 0010 swap rd
1001 010d dddd 0011 inc rd
1001 010d dddd 0100 (reserved)
1001 010d dddd 0101 asr rd
1001 010d dddd 0110 lsr rd
1001 010d dddd 0111 ror rd

* zero-operand instructions:

1001 0100 bbbb 1000 sex/clx status register clear/set bit
1001 0101 0000 1000 ret
1001 0101 0001 1000 reti
1001 0101 001x 1000 (reserved)
1001 0101 01xx 1000 (reserved)
1001 0101 1000 1000 sleep
1001 0101 1001 1000 break
1001 0101 1010 1000 wdr
1001 0101 1011 1000 (reserved)
1001 0101 110q 1000 lpm/elpm
1001 0101 1110 1000 spm
1001 0101 1111 1000 spm z+
1001 010c 000e 1001 indirect jump/call to z or eind:z
1001 010d dddd 1010 dec rd
1001 0100 kkkk 1011 des round k

* misc (2)

1001 010k kkkk 11ck kkkk kkkk kkkk kkkk jmp/call abs22
1001 0110 kkpp kkkk adiw rp,uimm6
1001 0111 kkpp kkkk sbiw rp,uimm6
1001 10b0 aaaa abbb cbi/sbi a,b (clear/set i/o bit)
1001 10b1 aaaa abbb sbic/sbis a,b (i/o bit test)
1001 11rd dddd rrrr mul, unsigned: r1:r0 = rr * rd
1011 saad dddd aaaa in/out to i/o space
110c ssss ssss ssss rjmp/rcall to pc + simm12
1110 kkkk dddd kkkk ldi rd,k
1111 0bss ssss sbbb conditional branch on status register bit
1111 10sd dddd 0bbb bld/bst register bit to status.t
1111 11bd dddd 0bbb sbrc/sbrs skip if register bit equals b 
