#define F_CPU 1000000UL
#include <avr/io.h>
#include <util/delay.h>

int main (void) {
    PORTB = 0;
    DDRB |= _BV(DDB0); // Set PORT B0 as output
    while(1) {
        PORTB ^= _BV(PB0); // Toggle PORT B0
        // _delay_ms max value: 262.14 ms / F_CPU in MHz = 262.14
        _delay_ms(100);
    }
}
