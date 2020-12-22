# Board Support Crate for STM32F401 (Black Pill) Development Board

**Nothing's been finished yet**...
this is a horrible mish-mash of Johan's Nucleo crate and my changes.

## Running the examples

1. Clone this repository
2. Flash an example
    1. Using [Probe.rs](https://crates.io/crates/probe-rs)

        ```cargo flash --chip stm32f401cc --example button-interrupt```

    2. Using [cargo embed](https://crates.io/crates/cargo-embed)

        ```cargo embed --release --example button-rtic```

## Board properties

* User led on PC13
* User button on PA0
* Serial port through ST-LINK on USART2, Tx: PA2 and Rx: PA3.

This repository is forked from
[jkristell/nucleo-f401re](https://github.com/jkristell/nucleo-f401re)
which, in turn, was based on
[therealprof/stm32f407g-disc](https://github.com/therealprof/stm32f407g-disc)
