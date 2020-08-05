# Examples

Small programs to be run on the simulator

## Compiler instalation

~~~{.bash}
sudo apt install gcc-avr avr-libc
~~~

## Useful commands

* Compile a source file

~~~{.bash}
avr-gcc -mmcu=avr2 -nostartfiles file.c -o file.o
~~~

* Convert into ihex / binary file

~~~
avr-objcopy -j .text -j .data -O ihex file.o file.hex
avr-objcopy -j .text -j .data -O binary file.o file.bin
~~~

* Disassembly

~~~
avr-objdump -m avr2 -D file-to-dissasemble.hex
~~~

* Convert from binary to ihex

~~~
avr-objcopy -I binary -O ihex file.bin file.hex
~~~
