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
    
    // 7-bit i2c bus address of accelerometer.
    const ACC_ADDR: u8 = 0b0011001;

    // Register for checking identification and operation.
    const WHO_AM_I_A: u8 = 0x0f;

    // Register for reading temperature "low
    // bits". Apparently unused per data sheet, but must be
    // read as part of a temperature read.
    const OUT_TEMP_L_A: u8 = 0x0c;

    // The temperature is read by a two-byte read from the
    // low register, so this register value is not needed.
    //   const OUT_TEMP_H_A: u8 = 0x0d;

    // Register for reading temperature value status.
    const STATUS_REG_AUX_A: u8 = 0x07;
    // Status bit for temperature overrun.
    const TOR: u8 = 0x40;
    // Status bit for new temperature data available.
    const TDA: u8 = 0x04;

    // Register for configuring the temperature unit.
    const TEMP_CFG_REG_A: u8 = 0x1f;
    // Bits to enable the temperature unit.
    const TEMP_EN: u8 = 0xc0;

    // Register for enabling the accelerometer.
    const CTRL_REG1_A: u8 = 0x20;
    // Bits to set a 1Hz data rate and "normal" mode,
    // leaving the actual accelerometer turned off.
    const DATA_RATE_1HZ: u8 = 0x10;

    // Register for controlling accelerometer data transfer.
    const CTRL_REG4_A: u8 = 0x23;
    // Bits to set "block data update" mode only.
    const BDU: u8 = 0x80;

    // Check that things are OK.
    let payload = [WHO_AM_I_A];
    let mut who_am_i = [0];
    i2c.write_read(ACC_ADDR, &payload, &mut who_am_i).unwrap();
    assert_eq!(0x33, who_am_i[0]);

    // Configure the accelerometer with just the temperature
    // unit enabled and running.
    let payload = [CTRL_REG1_A, DATA_RATE_1HZ];
    i2c.write(ACC_ADDR, &payload).unwrap();
    let payload = [TEMP_CFG_REG_A, TEMP_EN];
    i2c.write(ACC_ADDR, &payload).unwrap();
    let payload = [CTRL_REG4_A, BDU];
    i2c.write(ACC_ADDR, &payload).unwrap();

    loop {
        // Get the temperature unit status.
        let payload = [STATUS_REG_AUX_A];
        let mut status = [0];
        i2c.write_read(ACC_ADDR, &payload, &mut status).unwrap();
        let overrun = (status[0] & TOR) > 0;
        let new_data = (status[0] & TDA) > 0;

        // Read the low and high temperature bytes as a
        // two-word transaction.
        let mut temp = [0, 0];
        // Apparently the high bit needs to be set for a two-word
        // read. Thanks to the lgm303agr crate for this.
        let payload = [OUT_TEMP_L_A | 0x80];
        i2c.write_read(ACC_ADDR, &payload, &mut temp).unwrap();

        // Convert the temperature data.
        //
        // Start by swizzling the data into a form where the
        // registers are combined and then treated as a
        // signed fixed-point offset.
        //
        // The datasheet says "Temperature data is stored
        // inside OUT_TEMP_H as // two’s complement data in
        // 8-bit format left-justified." Apparently
        // OUT_TEMP_L is also significant, as expected.
        let temp = (((temp[1] as u16) << 8) | temp[0] as u16) as i16;
        // Convert to fixed-point as a float.
        // Apparently the temperature needs to be offset by
        // 25°C. Thanks to the lgm303agr crate for this.
        let deg_c = temp as f32 / 256.0 + 25.0;
        let deg_f = deg_c * 9.0 / 5.0 + 32.0;

        // Display the data.
        rprint!("{}", deg_f);
        if overrun {
            rprint!(" (overrun)");
        }
        if !new_data {
            rprint!(" (old data)");
        }
        rprintln!();

        // New thermometer data won't be available for a
        // while, so wait for it.
        timer.delay_ms(1000u16);
    }
}
