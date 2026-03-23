# mb2-thermometer: display current (chip) temperatures
Bart Massey 2023

This code prints the current temperature of chips on the
MicroBit v2 in °F or °C once per second and it also shows a rough
temperature value using the 5 x 5 LED display. The user can press either button 
on the MicroBit v2 to change the measurement setting. The LSM303AGR
temperature unit and the CPU temperature unit are supported.

* The `main` branch uses the `lsm303agr` crate for the LSM303AGR.

* The `twim` branch does raw i2c for the LSM303AGR to
  demonstrate that process.

## Build and Run

Compile and run with `cargo embed --release`. To compile and run in debug mode, type `cargo embed`.

## What was changed

Please see this [pull request](https://github.com/pdx-cs-rust-embedded/mb2-thermometer/pull/1) for additional details.

## How it went

Lee Hoang: It went pretty well. I consider the changes made to this project to be a good exercise in reading source code, looking up documentation, and applying comments where appropriate. 

## Acknowledgements

Thanks to the authors of the `lsm303agr` crate. It is a
really nice interface to that device.

While this code was written without reference to the
examples of accelerometer and magnetometer use in the Rust
Embedded Discovery Book for the Microbit v2, those examples
were definitely the inspiration for writing the initial
version of this.

Another acknowledgement to Claude Code for suggesting code improvements 
that cargo clippy missed.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
