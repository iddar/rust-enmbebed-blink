#![no_std]
#![no_main]

use nb::{self, block};
use void::{unreachable, Void};
extern crate panic_halt;
use cortex_m_rt::entry;

// extern crate nrf52840_hal;
// use nrf52840_hal::prelude::*;
// use nrf52840_hal::pac::Peripherals;
// use nrf52840_hal::gpio::*;
// use nrf52840_hal::Timer;


extern crate nrf52840_pac as pac;
extern crate embedded_hal as hal;
// use hal::blocking::delay::DelayMs;
// use embedded_hal::digital::v2::OutputPin;

// pub fn delay(&mut self, cycles: u32) {
//     self.start(cycles);
//     match block!(self.wait()) {
//         Ok(_) => {}
//         Err(x) => unreachable(x),
//     }
// }

fn wait<'a>(timer: &'a pac::TIMER0) -> nb::Result<(), Void> {
    if timer.events_compare[0].read().bits() == 0 {
        // EVENTS_COMPARE has not been triggered yet
        return Err(nb::Error::WouldBlock);
    }

    // Reset the event, otherwise it will always read `1` from now on.
    timer.events_compare[0].write(|w| w);

    Ok(())
}

fn start<'a>(timer: &'a pac::TIMER0, cycles: u32) {
    timer.events_compare[0].reset();
    timer.cc[0].write(|w|
        // The timer mode was set to 32 bits above, so all possible values
        // of `cycles` are valid.
        unsafe { w.cc().bits(cycles as u32) }
    );

    // Clear the counter value.
    timer.tasks_clear.write(|w|
        unsafe { w.bits(1) }
    );

    // Start the timer.
    timer.tasks_start.write(|w|
        unsafe { w.bits(1) }
    );
}

fn delay<'a>(timer: &'a pac::TIMER0, cycles: u32) {
    start(&timer, cycles);
    match block!(wait(&timer)) {
        Ok(_) => {}
        Err(x) => unreachable(x),
    }
}

fn init<'a>(nrf: &'a pac::Peripherals) -> pac::TIMER0 {
    nrf.TIMER0.shorts
        .write(|w|
            w.compare0_clear().enabled().compare0_stop().enabled()
        );
    nrf.TIMER0.prescaler.write(
        |w| unsafe { w.prescaler().bits(4) }, // 1 MHz
    );
    nrf.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
}

#[entry]
fn main() -> ! {
    // let board = Peripherals::take().unwrap();
    // let p0 = p0::Parts::new(board.P0);
    // let mut led1 = p0.p0_24
    //     .into_push_pull_output(Level::High);

    // let mut timer = Timer::new(board.TIMER0);
    let nrf = pac::Peripherals::take().unwrap();
    init(&nrf);

    let mut ptr = pac::P0::ptr();
    
    unsafe {
        nrf.P0.pin_cnf[24].write(|w| {
            w.dir().output();
            w.input().disconnect();
            w.pull().disabled();
            w.drive().s0s1();
            w.sense().disabled();
            w
        });
    }

    unsafe {
        (ptr).outclr.write(|w| {
            w.bits(1u32 << 24);
        });
    }

    loop {
        delay(&nrf.TIMER0, 3_000_000);
        // led1.set_low().unwrap();
        // timer.delay(3_000_000);
        // led1.set_high().unwrap();
        // timer.delay(1_000_000);
    }
}