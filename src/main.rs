#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
extern crate panic_halt;
extern crate nrf52840_hal;
use cortex_m_rt::entry;

use nrf52840_hal::prelude::*;
use nrf52840_hal::gpio::*;
use nrf52840_hal::Timer;
use nrf52840_hal::pac as nrf52;

use nrf52::{
    CorePeripherals,
    Peripherals,
    Interrupt,
    interrupt
};

#[interrupt]
fn TIMER0() {
    nrf52::NVIC::unpend(Interrupt::TIMER0);
    TICK.store(true, Ordering::Relaxed);
}

static TICK: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    // let core = CorePeripherals::take().unwrap();
    let board = Peripherals::take().unwrap();
    let p0 = p0::Parts::new(board.P0);
    let mut led1 = p0.p0_24
        .into_push_pull_output(Level::High);

    let mut timer = Timer::new(board.TIMER0);
    
    timer.enable_interrupt();
    // let nvic = &mut core.NVIC;

    unsafe {
        nrf52::NVIC::unmask(Interrupt::RTC0);
        // nvic.set_priority(Interrupt::TIMER0, 1);
    }

    let mut led_on = false;
    loop {
        if TICK.swap(false, Ordering::Relaxed) {
            led_on = !led_on;
            if led_on {
                led1.set_high().unwrap();
            } else {
                led1.set_low().unwrap();
            }
        }
    }
}