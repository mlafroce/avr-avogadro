#define F_CPU 1000000UL
#include <avr/io.h>
#include <util/delay.h>

int main (void) {
    DDRB |= _BV(DDB0); // Set PORT B0 as output
    while(1) {
        PORTB ^= _BV(PB0); // Toggle PORT B0
        _delay_ms(500);
    }
}
