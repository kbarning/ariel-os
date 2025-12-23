all: main.bin

# Übersetzten in eine Objektdatei
main.o: main.c
	arm-none-eabi-gcc -mcpu=cortex-m33 -mthumb \
           -fPIC -Os -c main.c -o main.o

# Den Binärcode extrahieren
main.bin: main.o
	arm-none-eabi-objcopy -O binary --only-section=.text main.o main.bin

clean:
	rm -f main.o main.bin

.PHONY: all clean