#![no_std]
#![no_main]

use arduino_nano_connect::hal::fugit::HertzU32;
use panic_halt as _;
use arduino_nano_connect as bsp;
use bsp::pac;
use bsp::hal;

use core::fmt::Write;
use hal::clocks::Clock;

use hal::uart::{DataBits, StopBits, UartConfig};

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

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        pins.gpio0.into_function(),
        pins.gpio1.into_function(),
    );

    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(HertzU32::Hz(115200), DataBits::Eight, None, StopBits::One), 
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    uart.write_full_blocking(b"UART example\r\n");

    let mut value = 0u32;
    loop {
        writeln!(uart, "value: {value:02}\r").unwrap();
        delay.delay_ms(1000);
        value += 1;
    }
}