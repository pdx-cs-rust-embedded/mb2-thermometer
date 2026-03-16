#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use cortex_m_rt::entry;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board,
    hal::{prelude::*, Temp, Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board and peripherals.
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    );
    let mut timer = Timer::new(board.TIMER0);
    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    
    // Initialize the accelerometer.
    lsm303.init().unwrap();
    lsm303.set_accel_mode_and_odr(
        &mut timer,
        lsm303agr::AccelMode::Normal,
        lsm303agr::AccelOutputDataRate::Hz1,
    ).unwrap();

    let mut nrf_temp = Temp::new(board.TEMP);

    loop {
        // Get the temperature status and degrees in Celsius.
        let status = lsm303.temperature_status().unwrap();
        let deg_c = lsm303.temperature().unwrap().degrees_celsius();

        // Show the temperature in Fahrenheit.
        // If there is an overrun or new data is not available,
        // show the appropriate notices.
        rprint!("acc: {}", deg_c * 9.0 / 5.0 + 32.0); 
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
        rprintln!("cpu: {}", deg_f);

        rprintln!();
        timer.delay_ms(1000u16);
    }
}
