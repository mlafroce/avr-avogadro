# Development status

**Version**: Minimal AVR1 Core

**Supported by**: AT90S1200, ATtiny11, ATtiny12, ATtiny15, ATtiny28

**Instructions**:

* **Arithmetic**:

* [x] `ADD`: Add without carry
* [x] `ADC`: Add with carry
* [x] `SUB`: Substract without carry 
* [x] `SUBI`: Substract immediate without carry
* [x] `SBC`: Substract with carry
* [x] `SBCI`: Substract with carry, set bit on I/O registry

* **Logic**
* [x] `AND`: Logical *AND*
* [x] `ANDI`: Logical *AND* with immediate
* [x] `OR`: Logical *OR*
* [x] `ORI`: Logical *OR* with immediate
* [x] `EOR`: Logical *XOR* (exclusive or)
* [ ] `COM`: One's complement
* [ ] `NEG`: Two's complement
* [ ] `SBR`: Set bits in register
* [ ] `CBR`: Clear bits in register
* [ ] `INC`: Increment
* [ ] `DEC`: Decrement
* [ ] `TST`: Test for zero or minus
* [ ] `CLR`: Clear register
* [ ] `SER`: Set all bits in register

* **Comparison**
* [x] `CP`: Compare
* [x] `CPC`: Compare with carry
* [x] `CPI`: Compare with immediate


* **Call / Jumps**

* [x] `RJMP`: Relative jump
* [x] `RCALL`: Relative call to subroutine
* [x] `RET`: Return from subroutine
* [x] `RETI`: Return from interrupt


* **Skips**

* [x] `CPSE`: Compare skip if equals
* [ ] `SBIC`: Skip if bit in I/O register is cleared
* [ ] `SBIS`: Skip if bit in I/O register is set
* [ ] `SBRC`: Skip if bit in register is cleared
* [ ] `SBRS`: Skip if bit in register is set


* **Branches**

* [x] `BRBC`: Branch if bit in SREG is cleared
* [x] `BRBS`: Branch if bit in SREG is set
* [x] `BREQ`: Branch if equal
* [x] `BRNE`: Branch if not equal
* [x] `BRCC`: Branch if carry cleared
* [x] `BRCS`: Branch if carry set
* [x] `BRSH`: Branch if same or higher
* [x] `BRLO`: Branch if lower (unsigned)
* [x] `BRMI`: Branch if minus
* [x] `BRPL`: Branch if plus
* [x] `BRGE`: Branch if greater or equal
* [x] `BRLT`: Branch if less than (signed)
* [x] `BRHC`: Branch if half carry is cleared
* [x] `BRHS`: Branch if half carry is set
* [x] `BRTC`: Branch if T flag is cleared
* [x] `BRTS`: Branch if T flag is set
* [x] `BRVS`: Branch if overflow is set
* [x] `BRVC`: Branch if overflow is cleared
* [x] `BRIE`: Branch if global interrupt is enabled
* [x] `BRID`: Branch if global interrupt is disabled

* **Transfers**:
* [x] `LD`:
* [x] `ST`:
* [x] `MOV`,
* [x] `LDI`:
* [x] `IN`:
* [x] `OUT`:
* [ ] `LPM`:

* **Bitwise**:

* [ ] `SBI`: Set bit in I/O register
* [ ] `CBI`: Clear bit in I/O register
* [ ] `LSL`: Logical shift left
* [ ] `LSR`: Logical shift right
* [ ] `ROL`: Rotate left through carry
* [ ] `ROR`: Rotate right through carry
* [ ] `ASR`: Arithmetic shift right:
* [ ] `SWAP`: Swap nibbles

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

* [x] `NOP`: No operation
* [ ] `SLEEP`
* [ ] `WDR`: Watchdog reset


**Extra**: Instructions from AVR2 family

* [x] `PUSH`: Push register on stack
* [x] `POP`: Pop register from stack