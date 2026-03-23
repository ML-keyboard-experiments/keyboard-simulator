#![no_std]
#![no_main]

use core::convert::Infallible;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use hal::gpio::Level;
use nrf52840_hal as hal;
use panic_halt as _;

const PRESSED_TIME: u32 = 10;

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
    // next song
    let mut button1 = port0.p0_04.into_push_pull_output(Level::Low).degrade();
    // [4]
    // mute
    let mut button4 = port0.p0_28.into_push_pull_output(Level::Low).degrade();
    // [3]
    // play
    let mut button3 = port0.p0_29.into_push_pull_output(Level::Low).degrade();
    // [2]
    // prev song
    let mut button2 = port0.p0_03.into_push_pull_output(Level::Low).degrade();

    let mut trigger_pin = port0.p0_05.into_push_pull_output(Level::High);

    let mut buttons: [&mut dyn OutputPin<Error = Infallible>; 4] =
        [&mut button1, &mut button2, &mut button3, &mut button4];

    // Get access to the core peripherals for delay
    let core = hal::pac::CorePeripherals::take().unwrap();
    let mut delay = hal::delay::Delay::new(core.SYST);

    // indicate start of simulation
    delay.delay_ms(100);
    trigger_pin.set_low().unwrap();

    // 8 h: idle
    simulate_idle(&mut delay, 60 * 1000 * 8);
    // 2 h: heavy usage
    simulate_heavy_typing(&mut buttons, &mut delay, 60 * 1000 * 2);
    // 2 h: idle
    simulate_idle(&mut delay, 60 * 1000 * 2);
    // 2 h: heavy usage
    simulate_heavy_typing(&mut buttons, &mut delay, 60 * 1000 * 2);
    // 3 h: light usage
    simulate_light_typing(&mut buttons, &mut delay, 60 * 1000 * 3);
    // 3 h: idle
    simulate_idle(&mut delay, 60 * 1000 * 3);
    // 2 h: light usage
    simulate_light_typing(&mut buttons, &mut delay, 60 * 1000 * 2);
    // 2 h: idle
    simulate_idle(&mut delay, 60 * 1000 * 2);

    // indicate end of simulation
    trigger_pin.set_high().unwrap();

    panic!();
}

fn simulate_idle(delay: &mut hal::delay::Delay, duration: u32) {
    delay.delay_ms(duration);
}

fn simulate_light_typing(
    buttons: &mut [&mut dyn OutputPin<Error = Infallible>],
    delay: &mut hal::delay::Delay,
    duration: u32,
) {
    let typing_speed: u32 = 125; // in key presses per minute
    simulate_typing(buttons, delay, duration, typing_speed);
}

fn simulate_heavy_typing(
    buttons: &mut [&mut dyn OutputPin<Error = Infallible>],
    delay: &mut hal::delay::Delay,
    duration: u32,
) {
    let typing_speed: u32 = 250; // in key presses per minute
    simulate_typing(buttons, delay, duration, typing_speed);
}

fn simulate_typing(
    buttons: &mut [&mut dyn OutputPin<Error = Infallible>],
    delay: &mut hal::delay::Delay,
    duration: u32,
    typing_speed: u32,
) {
    let pause_time: u32 = (1000 * 60) / typing_speed - PRESSED_TIME; // in ms
    let cycle_time: u32 = PRESSED_TIME + pause_time;
    let mut elapsed: u32 = 0;

    loop {
        for button in buttons.iter_mut() {
            if elapsed >= duration {
                return;
            }
            simulate_key_press(button, delay);
            delay.delay_ms(pause_time);
            elapsed += cycle_time;
        }
    }
}

fn simulate_key_press(button: &mut impl OutputPin, delay: &mut hal::delay::Delay) {
    button.set_high().unwrap();
    delay.delay_ms(PRESSED_TIME);
    button.set_low().unwrap();
}
