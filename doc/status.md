# Development status

**Reference**:

* [ ] Not implemented
* [-] Not tested / tests incomplete
* [x] Tested

## Instruction set

### Minimal AVR1 Core

**Supported by**: AT90S1200, ATtiny11, ATtiny12, ATtiny15, ATtiny28

* **Arithmetic**:

* [x] `ADD`: Add without carry
* [x] `ADC`: Add with carry
* [x] `SUB`: Substract without carry 
* [-] `SUBI`: Substract immediate without carry
* [-] `SBC`: Substract with carry
* [-] `SBCI`: Substract with carry, set bit on I/O registry

* **Logic**
* [-] `AND`: Logical *AND*
* [-] `ANDI`: Logical *AND* with immediate
* [-] `OR`: Logical *OR*
* [-] `ORI`: Logical *OR* with immediate
* [-] `EOR`: Logical *XOR* (exclusive or)
* [-] `COM`: One's complement
* [-] `NEG`: Two's complement
* [-] `SBR`: Set bits in register (meta-op)
* [-] `CBR`: Clear bits in register (meta-op)
* [-] `INC`: Increment
* [-] `DEC`: Decrement
* [-] `TST`: Test for zero or minus (meta-op)
* [-] `CLR`: Clear register (meta-op)
* [-] `SER`: Set all bits in register (meta-op)

* **Comparison**
* [-] `CP`: Compare
* [-] `CPC`: Compare with carry
* [-] `CPI`: Compare with immediate


* **Call / Jumps**

* [-] `RJMP`: Relative jump
* [-] `RCALL`: Relative call to subroutine
* [-] `RET`: Return from subroutine
* [-] `RETI`: Return from interrupt


* **Skips**

* [-] `CPSE`: Compare skip if equals
* [-] `SBIC`: Skip if bit in I/O register is cleared
* [-] `SBIS`: Skip if bit in I/O register is set
* [ ] `SBRC`: Skip if bit in register is cleared
* [ ] `SBRS`: Skip if bit in register is set


* **Branches**

* [-] `BRBC`: Branch if bit in SREG is cleared
* [-] `BRBS`: Branch if bit in SREG is set
* [-] `BREQ`: Branch if equal
* [-] `BRNE`: Branch if not equal
* [-] `BRCC`: Branch if carry cleared
* [-] `BRCS`: Branch if carry set
* [-] `BRSH`: Branch if same or higher
* [-] `BRLO`: Branch if lower (unsigned)
* [-] `BRMI`: Branch if minus
* [-] `BRPL`: Branch if plus
* [-] `BRGE`: Branch if greater or equal
* [-] `BRLT`: Branch if less than (signed)
* [-] `BRHC`: Branch if half carry is cleared
* [-] `BRHS`: Branch if half carry is set
* [-] `BRTC`: Branch if T flag is cleared
* [-] `BRTS`: Branch if T flag is set
* [-] `BRVS`: Branch if overflow is set
* [-] `BRVC`: Branch if overflow is cleared
* [-] `BRIE`: Branch if global interrupt is enabled
* [-] `BRID`: Branch if global interrupt is disabled

* **Transfers**:
* [-] `LD`:
* [-] `ST`:
* [-] `MOV`,
* [-] `LDI`:
* [-] `IN`:
* [-] `OUT`:
* [ ] `LPM`:

* **Bitwise**:

* [-] `SBI`: Set bit in I/O register
* [-] `CBI`: Clear bit in I/O register
* [ ] `LSL`: Logical shift left
* [-] `LSR`: Logical shift right
* [ ] `ROL`: Rotate left through carry
* [-] `ROR`: Rotate right through carry
* [-] `ASR`: Arithmetic shift right:
* [-] `SWAP`: Swap nibbles

* **Status register**

* [ ] `BSET`: Bit set in SREG
* [ ] `BCLR`: Bit clear in SREG
* [ ] `BST`: Bit store from Bit in Register to T flag
* [ ] `BLD`: Bit load from T flag in SREG to a bit in Register
* [ ] `SEC`: Set carry flag
* [ ] `CLC`: Clear carry flag
* [ ] `SEN`: Set negative flag
* [ ] `CLN`: Clear negative flag
* [ ] `SEZ`: Set zero flag
* [ ] `CLZ`: Clear zero flag
* [ ] `SEI`: Set global interrupt flag
* [ ] `CLI`: Clear global interrupt flag
* [ ] `SES`: Set signed flag
* [ ] `CLS`: Clear signed flag
* [ ] `SEV`: Set overflow flag
* [ ] `CLV`: Clear overflow flag
* [ ] `SET`: Set T flag
* [ ] `CLT`: Clear T flag
* [ ] `SEH`: Set half carry flag
* [ ] `CLH`: Clear half carry flag

* **Special**

* [-] `NOP`: No operation
* [ ] `SLEEP`
* [ ] `WDR`: Watchdog reset


### Minimal AVR2 Core

**Supported by**: AT90S2313, AT90S2323, ATtiny22, AT90S2333, AT90S2343, AT90S4414, AT90S4433, AT90S4434, AT90S8515, AT90C8534, AT90S8535, ATtiny26

* **Arithmetic**:

* [-] `ADIW`: Add immediate to word
* [-] `SBIW`: Substract immediate from word

* **Transfers**:

* [-] `LD`: LD X, LD Y + k, LD Z + k
* [-] `ST`: ST X, ST Y + k, ST Z + k
* [-] `PUSH`: Push register on stack
* [-] `POP`: Pop register from stack
