# Board Support Crate for STM32F401 (Black Pill) Development Board

**Nothing's been finished yet**...
this is a horrible mish-mash of Johan's Nucleo crate and my changes.

## Building the examples

After cloning this repository

```cargo build --release --chip stm32f401cc --example button-interrupt```

You should then  find an ELF binary in
```target/thumbv7em-none-eabihf/release/examples/```

## Flashing the examples

This can be done from within Cargo using
[cargo-flash](https://github.com/probe-rs/cargo-flash)
or
[cargo-embed](https://github.com/probe-rs/cargo-embed)

There's also various uploaders available from outside the Rust ecosystem.

*Just to complicate matters further I'm using a
[Black Magic Probe](https://github.com/rust-embedded/debugonomicon/blob/master/src/overview.md#black-magic-probe)

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
