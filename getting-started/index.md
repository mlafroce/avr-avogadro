---
layout: page
title: Getting started
---

# Loading a file

## Building binary files

*AVR-Avogadro* supports compiled **binary files**. You can create them using *avr-gcc* or even *Arduino IDE*.

### Using *avr-gcc*

Lets build a simple blink application, copy this into a file named `blink.c`.

~~~{.c}
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
~~~

We can compile it with `avr-gcc` using command line like this

~~~
avr-gcc -Wall -mmcu=attiny85 blink.c -o blink.o
avr-objcopy -j .text -j .data -O binary blink.o blink.bin
~~~

There are some examples in the *examples* folder.

### Using *Arduino ide*

With Arduino IDE 1.6.5 or later, you can go to *Sketch* -> *Export compiled library* to generate an *hex* file.

Currently this format is not supported, but can be transformed using command line:

~~~
avr-objcopy -I ihex -O binary <file>.hex <file>.bin
~~~

Where `file` is the name of the project
