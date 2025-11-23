# Generated with ChatGPT

# ------------------------------------------------------------
# Cortex-M33 (ARMv8-M) raw Thumb-2 function builder
# Produces: main.bin   (raw .text section only)
# ------------------------------------------------------------

CC      := arm-none-eabi-gcc
OBJCOPY := arm-none-eabi-objcopy
OBJDUMP := arm-none-eabi-objdump

CFLAGS  := -mcpu=cortex-m33 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard \
           -fPIC -Os -Wall -Wextra -ffreestanding -nostdlib

all: main.bin main.dis

# Compile C → object
main.o: main.c
	$(CC) $(CFLAGS) -c main.c -o main.o

# Extract ONLY the .text section → raw machine code
main.bin: main.o
	$(OBJCOPY) -O binary --only-section=.text main.o main.bin
	@echo "Generated main.bin (raw function bytes)"

# Optional: disassemble the object for inspection
main.dis: main.o
	$(OBJDUMP) -d main.o > main.dis

clean:
	rm -f main.o main.bin main.dis

.PHONY: all clean