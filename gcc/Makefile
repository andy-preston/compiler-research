LINKER_SCRIPT = linker_script.ld
C_FLAGS  = -c -O0 -g3 -mthumb -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=hard

all: main.elf

%.o: %.c
	arm-none-eabi-gcc $(C_FLAGS) -o $@ $<

main.elf: main.o
	arm-none-eabi-ld -T$(LINKER_SCRIPT) -o main.elf main.o

clean:
	rm -rf *.o *.elf

.PHONY: dump
dump:
	arm-none-eabi-objdump -d -s main.elf
