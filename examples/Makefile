CC=avr-gcc
MMCU=attiny85
ARCH=avr2
OBJCOPY=avr-objcopy

SOURCES ?= $(wildcard *.c *.s) 
BIN_FILES_AUX = $(patsubst %.c,%.bin,$(SOURCES))
BIN_FILES = $(patsubst %.s,%.bin,$(BIN_FILES_AUX))
HEX_FILES = $(patsubst %.bin,%.hex,$(BIN_FILES))

all: hex bin

bin: $(BIN_FILES)

hex: $(HEX_FILES)

%.o: %.s
	$(CC) -Wall -mmcu=$(MMCU) -nostartfiles $< -o $@

01-sum.o: 01-sum.c
	$(CC) -Wall -mmcu=$(MMCU) -nostartfiles $< -o $@

03-blink.o: 03-blink.c
	$(CC) -Wall -mmcu=$(MMCU) -O2 $< -o $@

%.o: %.c
	$(CC) -Wall -mmcu=$(MMCU) $< -o $@

%.bin: %.o
	$(OBJCOPY) -j .text -j .data -O binary $< $@

%.hex: %.o
	$(OBJCOPY) -j .text -j .data -O ihex $< $@

clean:
	$(RM) *.o *.bin *.hex

.PHONY: all bin clean hex
