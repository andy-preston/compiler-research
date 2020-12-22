# Board Support Crate for STM32F401 (Black Pill) Development Board

Based on
[jkristell/nucleo-f401re](https://github.com/jkristell/nucleo-f401re)

**Nothing's been finished yet**...
this is a horrible mish-mash of Johan's Nucleo crate and my changes.

## Running the examples

1. Clone this repository

### Flash using Probe.rs

```cargo flash --chip stm32f401re --example button-interrupt```

Or with cargo embed

```cargo embed --release --example button-rtic```

If probe fails to flash your board you probably need to update the firmware on the onboard programmer.
The updater can be found at: https://www.st.com/en/development-tools/stsw-link007.html

## Board properties

 * User led on PA5
 * Serial port through ST-LINK on USART2, Tx: PA2 and Rx: PA3.
* User button on PA0

This repository is based on https://github.com/therealprof/stm32f407g-disc
