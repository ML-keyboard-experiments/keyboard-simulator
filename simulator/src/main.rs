#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use hal::gpio::Level;
use nrf52840_hal as hal;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let p = hal::pac::Peripherals::take().unwrap();

    // Split the GPIO port into individual pins
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    //    ┌───────────────────────────────────────────────────┐
    //    │                                                   │
    //    │   ┌────────────┐               ┌─────────────┐    │
    //    │   │  KEYBOARD  │               │  SIMULATOR  │    │
    //    │   │  XIAO NRF  │               │  XIAO NRF   │    │
    //    │   └────────────┘               └─────────────┘    │
    //    │                                                   │
    //    ├──────────────────┬────────────────────────────────┘
    //    │                  │
    //    │                  │
    //    │                  │
    //    │                  │
    //    │   [1]      [2]   │
    //    │                  │
    //    │   [3]      [4]   │
    //    │                  │
    //    └──────────────────┘

    // [1]
    let mut next_song = port0.p0_04.into_push_pull_output(Level::High);
    // [4]
    let mut mute = port0.p0_28.into_push_pull_output(Level::High);
    // [3]
    let mut play = port0.p0_29.into_push_pull_output(Level::High);
    // [2]
    let mut prev_song = port0.p0_03.into_push_pull_output(Level::High);

    // Get access to the core peripherals for delay
    let core = hal::pac::CorePeripherals::take().unwrap();
    let mut delay = hal::delay::Delay::new(core.SYST);

    loop {
        play.set_high().unwrap();
        delay.delay_ms(50_u32);
        play.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}
