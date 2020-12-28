# Board Support Crate for STM32F401 (Black Pill) Development Board

**Nothing's been finished yet**...
this is a horrible mish-mash of Johan's Nucleo crate and my changes.

## Building the examples

After cloning this repository

```cargo build --release --chip stm32f401cc --example button-interrupt```

You should then  find an ELF binary in
```target/thumbv7em-none-eabihf/release/examples/```

## Flashing the examples

If you modify ```.cargo/config``` to use whatever command you use for flashing
under ```runner=``` you can just use ```cargo run``` to compile and flash.

## Board properties

* STM32F401CC microcontroller
* User led on PC13
* User button on PA0
* Power supply
* Micro USB connector

This repository is forked from
[jkristell/nucleo-f401re](https://github.com/jkristell/nucleo-f401re)
which, in turn, was based on
[therealprof/stm32f407g-disc](https://github.com/therealprof/stm32f407g-disc)

## Contributing

Testing, improvements, bugs, fixes and documentation improvements
are all greatly appreciated.
