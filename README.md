# Board Support Crate for STM32F401 (Black Pill) Development Board

Based on
[jkristell/nucleo-f401re](https://github.com/jkristell/nucleo-f401re)

**Nothing's been finished yet**...
this is a horrible mish-mash of Johan's Nucleo crate and my changes.

## Running the examples

1. Clone this repository
2. Flash an example

### Using [Probe.rs](https://crates.io/crates/probe-rs)

```cargo flash --chip stm32f401re --example button-interrupt```

### Using [cargo embed](https://crates.io/crates/cargo-embed)

```cargo embed --release --example button-rtic```

## Board properties

* User led on PC13
* User button on PA0
* Serial port through ST-LINK on USART2, Tx: PA2 and Rx: PA3.

This repository is based on https://github.com/therealprof/stm32f407g-disc
