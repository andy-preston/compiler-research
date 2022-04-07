# Minimal STM32F401 Black Pill

This project is a very minimalist template project for STM32F4 microcontrollers.
It's "all batteries included" and requires no third party libraries/vendor code (excluding toolchain code).

Forked from https://github.com/samvrlewis/minimal-stm32

It's intended for the STM32F401 Black Pill Development Board but it should be
possible to modify it to run on other STM32 microcontrollers, with some
modifications to the linker script and register addresses.

The main.c that is currently included, doesn't actually *do* anything. It's
just some arbitrary floating point code to ensure the compiler is generating
instructions for the hardware floating point unit.

For simplicity, there's no support for global variables.

## Dependencies

### Compiler toolchain

To compile and link the `main.elf` application, you'll need the
[`arm-none-eabi`](https://launchpad.net/gcc-arm-embedded/+download)
compiler toolchain. A Dockerfile is included to provide this.

## Quick start

`./toolchain/container make` builds the `main.elf` application

`./toolchain/container dump` provides a disassembly listing of main.elf

`./toolchain/container make clean` removes built files
