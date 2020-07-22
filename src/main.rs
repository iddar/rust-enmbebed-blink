#![no_std]
#![no_main]

extern crate panic_halt;
extern crate nrf52840_hal;
use cortex_m_rt::entry;

use nrf52840_hal::prelude::*;
use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::gpio::*;
use nrf52840_hal::Timer;

#[entry]
fn main() -> ! {
    let board = Peripherals::take().unwrap();
    let p0 = p0::Parts::new(board.P0);
    let mut led1 = p0.p0_24
        .into_push_pull_output(Level::High);

    let mut timer = Timer::new(board.TIMER0);

    loop {
        led1.set_low().unwrap();
        timer.delay(3_000_000);
        led1.set_high().unwrap();
        timer.delay(1_000_000);
    }
}