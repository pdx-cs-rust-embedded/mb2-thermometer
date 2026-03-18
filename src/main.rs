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

// MB2 display - Celsius
const C_DISPLAY: [[u8; 5]; 5] = [
    [1, 0, 1, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 1, 1],
];

// MB2 display - Fahrenheit
const F_DISPLAY: [[u8; 5]; 5] = [
    [1, 0, 1, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board and peripherals, including the I2C, timer, and LSM303.
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);
    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    let mut display = Display::new(board.display_pins);
    let left_button = board.buttons.button_a;
    let right_button = board.buttons.button_b;

    // Determine if the user is viewing temps in Celsius or Fahrenheit
    let mut is_celsius = false;

    // Initialize the accelerometer.
    lsm303.init().unwrap();
    lsm303
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::Normal, // Normal power mode (10 bit)
            lsm303agr::AccelOutputDataRate::Hz1, // Output data rate is 1 Hz
        )
        .unwrap();

    let mut nrf_temp = Temp::new(board.TEMP);

    loop {
        // Get the temperature status and degrees in Celsius and Fahrenheit.
        let status = lsm303.temperature_status().unwrap();
        let deg_c = lsm303.temperature().unwrap().degrees_celsius();
        let deg_f1 = deg_c * 9.0 / 5.0 + 32.0;

        // if the left button is pressed, switch to Fahrenheit or Celsius based on the current setting
        if left_button.is_low().unwrap() && !is_celsius {
            is_celsius = true;
        } else if left_button.is_low().unwrap() && is_celsius {
            is_celsius = false;
        }

        // if the right button is pressed, switch to Fahrenheit or Celsius based on the current setting
        if right_button.is_low().unwrap() && !is_celsius {
            is_celsius = true;
        } else if right_button.is_low().unwrap() && is_celsius {
            is_celsius = false;
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
        if is_celsius {
            display.show(&mut timer, C_DISPLAY, 1000);
        } else {
            display.show(&mut timer, F_DISPLAY, 1000);
        }
    }
}
