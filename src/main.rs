#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use cortex_m_rt::entry;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, twim, Temp, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

// Convert the temperature value into an LED display
// that can be shown on the microbit.
// Due to LED size limitations, there are not enough lights to fully display a 2 digit number.
// Another limitation is that the right digit does not update. This may be fixed in the future.
//
// The middle column is blanked out to separate the two digits.
// The left two columns are for the second digit.
// The right two columns are for the first digit.
//
// For testing purposes, the current temperature range is 20 to 90 degrees.
//
// For reference: 32 degrees F is 0 degrees C
// 68 degrees F (room temp) is 20 degrees C
fn show_temp_val(temp_val: f32) -> [[u8; 5]; 5] {
    // Temperature is between 20 and 30 C or F
    if temp_val >= 20.0 && temp_val < 30.0 {
        return [
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [1, 0, 0, 1, 1],
            [1, 1, 0, 1, 1],
        ];
    // temperature is between 30 and 40 C or F
    } else if temp_val >= 30.0 && temp_val < 40.0 {
        return [
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
        ];
    // temperature is between 40 and 50 C or F
    } else if temp_val >= 40.0 && temp_val < 50.0 {
        return [
            [0, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
        ];
    // temperature is between 50 and 60 C or F
    } else if temp_val >= 50.0 && temp_val < 60.0 {
        return [
            [1, 1, 0, 1, 1],
            [1, 0, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
        ];

    // temperature is between 60 and 70 C or F
    } else if temp_val >= 60.0 && temp_val < 70.0 {
        return [
            [1, 1, 0, 1, 1],
            [1, 0, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
        ];

    // temperature is between 70 and 80 C or F
    } else if temp_val >= 70.0 && temp_val < 80.0 {
        return [
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 1],
            [1, 0, 0, 1, 1],
            [1, 0, 0, 1, 1],
        ];
    // temperature is between 80 and 90 C or F
    } else if temp_val >= 80.0 && temp_val < 90.0 {
        return [
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
        ];
    // temperature is not in the ranges above, show a blank LED display.
    } else {
        return [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board and peripherals, including the I2C, timer, and LSM303.
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);
    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    let mut display = Display::new(board.display_pins);
    let fahrenheit_button = board.buttons.button_a;
    let celsius_button = board.buttons.button_b;

    // Determine if the user is viewing temps in Celsius or Fahrenheit
    let mut is_celsius = false;

    // Initialize the accelerometer.
    lsm303.init().unwrap();

    let mut nrf_temp = Temp::new(board.TEMP);

    loop {
        // Get the temperature status and degrees in Celsius and Fahrenheit.
        let status = lsm303.temperature_status().unwrap();
        let deg_c = lsm303.temperature().unwrap().degrees_celsius();
        let deg_f1 = deg_c * 9.0 / 5.0 + 32.0; // store the Fahrenheit calculation in a variable

        // if the left button is pressed, switch to Fahrenheit.
        if fahrenheit_button.is_low().unwrap() && is_celsius {
            is_celsius = false;
        }

        // if the right button is pressed, switch to Celsius
        if celsius_button.is_low().unwrap() && !is_celsius {
            is_celsius = true;
        }

        // Show the temperature in Fahrenheit or Celsius.
        // If there is an overrun or new data is not available,
        // show the appropriate notices.
        if is_celsius {
            rprint!("acc: {}", deg_c);
        } else {
            rprint!("acc: {}", deg_f1);
        }
        if status.overrun() {
            rprint!(" (overrun)");
        }
        if !status.new_data() {
            rprint!(" (old data)");
        }
        rprintln!();

        // Use the CPU to measure temperature in Celsius and Fahrenheit.
        let deg_c: f32 = nrf_temp.measure().to_num();
        let deg_f = deg_c * 9.0 / 5.0 + 32.0;
        if is_celsius {
            rprintln!("cpu: {}", deg_c);
        } else {
            rprintln!("cpu: {}", deg_f);
        }

        rprintln!();

        // show the blocking display
        // with a temperature value.
        let mut temp_val = deg_c;
        if !is_celsius {
            temp_val = deg_f;
        }
        display.show(&mut timer, show_temp_val(temp_val), 1000);
    }
}
