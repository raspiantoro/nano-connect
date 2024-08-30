#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::digital::OutputPin;
use arduino_nano_connect as bsp;
use bsp::hal::prelude::*;
use bsp::hal::pac;
use bsp::hal;

#[arduino_nano_connect::entry]
fn main () -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ, 
        pac.XOSC, 
        pac.CLOCKS, 
        pac.PLL_SYS, 
        pac.PLL_USB, 
        &mut pac.RESETS, 
        &mut watchdog
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0, 
        pac.PADS_BANK0, 
        sio.gpio_bank0, 
        &mut pac.RESETS,
    );

    let mut led_pin = pins.sck0.into_push_pull_output();

    loop {
        let _ = led_pin.set_high().unwrap();
        delay.delay_ms(500);
        let _ = led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
