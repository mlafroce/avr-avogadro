CC=avr-gcc
MMCU=attiny13
ARCH=avr2
OBJCOPY=avr-objcopy

SOURCES ?= $(wildcard *.c *.s) 
BIN_FILES_AUX = $(patsubst %.c,%.bin,$(SOURCES))
BIN_FILES = $(patsubst %.s,%.bin,$(BIN_FILES_AUX))
HEX_FILES = $(patsubst %.bin,%.hex,$(BIN_FILES))

all: bin hex

bin: $(BIN_FILES)

hex: $(HEX_FILES)

%.o: %.c
	$(CC) -mmcu=$(MMCU) -nostartfiles -c $<

%.o: %.s
	$(CC) -mmcu=$(MMCU) -nostartfiles -c $<

%.bin: %.o
	$(OBJCOPY) -j .text -j .data -O binary $< $@

%.hex: %.o
	$(OBJCOPY) -j .text -j .data -O ihex $< $@

clean:
	$(RM) *.o *.bin *.hex

.PHONY: all bin clean hex
