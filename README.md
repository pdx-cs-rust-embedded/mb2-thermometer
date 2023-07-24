# mb2-thermometer: display current (chip) temperatures
Bart Massey 2023

This code prints the current temperature of the LSM303AGR
accelerometer/magnetometer/thermometer chip in °F once per
second.

* The `main` branch uses the `lsm303agr` crate.

* The `twim` branch does raw i2c to demonstrate that
  process.

## Acknowledgements

Thanks to the authors of the `lsm303agr` crate. It is a
really nice interface to that device.

While this code was written without reference to the
examples of accelerometer and magnetometer use in the Rust
Embedded Discovery Book for the Microbit v2, those examples
were definitely the inspiration for writing the initial
version of this.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
