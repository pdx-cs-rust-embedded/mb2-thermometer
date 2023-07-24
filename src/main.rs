#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};
use lsm303agr::Lsm303agr;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    );
    let mut timer = Timer::new(board.TIMER0);
    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    lsm303.init().unwrap();
    loop {
        let status = lsm303.temperature_status().unwrap();
        if status.overrun() {
            rprintln!("overrun");
        }
        if status.new_data() {
            rprintln!("new data");
        }
        let deg_c = lsm303.temperature().unwrap().degrees_celsius();
        rprintln!("{}", deg_c * 9.0 / 5.0 + 32.0);
        timer.delay_ms(1000u16);
    }
}
