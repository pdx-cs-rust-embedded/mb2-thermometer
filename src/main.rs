#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    );
    let mut timer = Timer::new(board.TIMER0);
    
    const ACC_ADDR: u8 = 0b0011001;
    const WHO_AM_I_A: u8 = 0x0f;
    const OUT_TEMP_L_A: u8 = 0x0c;
    //const OUT_TEMP_H_A: u8 = 0x0d;
    const STATUS_REG_AUX_A: u8 = 0x07;
    const TOR: u8 = 0x40;
    const TDA: u8 = 0x04;
    const TEMP_CFG_REG_A: u8 = 0x1f;
    const TEMP_EN: u8 = 0xc0;
    const CTRL_REG1_A: u8 = 0x20;
    const DATA_RATE_1HZ: u8 = 0x10;
    const CTRL_REG4_A: u8 = 0x23;
    const BDU: u8 = 0x80;

    let payload = [WHO_AM_I_A];
    let mut temp = [0];
    i2c.write_read(ACC_ADDR, &payload, &mut temp).unwrap();
    assert_eq!(0x33, temp[0]);

    let payload = [CTRL_REG1_A, DATA_RATE_1HZ];
    i2c.write(ACC_ADDR, &payload).unwrap();
    let payload = [TEMP_CFG_REG_A, TEMP_EN];
    i2c.write(ACC_ADDR, &payload).unwrap();
    let payload = [CTRL_REG4_A, BDU];
    i2c.write(ACC_ADDR, &payload).unwrap();

    loop {
        let payload = [STATUS_REG_AUX_A];
        let mut status = [0];
        i2c.write_read(ACC_ADDR, &payload, &mut status).unwrap();
        let overrun = (status[0] & TOR) > 0;
        let new_data = (status[0] & TDA) > 0;

        let mut temp = [0, 0];
        let payload = [0x80 | OUT_TEMP_L_A];
        i2c.write_read(ACC_ADDR, &payload, &mut temp).unwrap();

        // let deg_c = temp[1];
        //rprint!("{}", deg_c * 9.0 / 5.0 + 32.0);
        rprint!("{:x} {:x}", temp[0], temp[1]);
        if overrun {
            rprint!(" (overrun)");
        }
        if !new_data {
            rprint!(" (old data)");
        }
        rprintln!();
        timer.delay_ms(1000u16);
    }
}
