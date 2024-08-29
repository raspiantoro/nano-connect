#![no_std]
#![no_main]

use heapless::Vec;
use panic_halt as _;
use arduino_nano_connect as bsp;
use bsp::{pac, hal};
use bsp::hal::fugit::RateExtU32;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::UsbVidPid;
use core::fmt::Write;
use hal::clocks::Clock;
use hal::gpio::{FunctionI2C, Pin};
use lsm6dsox::*;

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

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(
        hal::usb::UsbBus::new(
            pac.USBCTRL_REGS, 
            pac.USBCTRL_DPRAM, 
            clocks.usb_clock, 
            true, 
            &mut pac.RESETS,
        )
    );

    let mut serial = usb_serial::UsbSerialBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd)).build().unwrap();

    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio12.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio13.reconfigure();

    let i2c = hal::I2C::i2c0(
        pac.I2C0, 
        sda_pin, 
        scl_pin, 
        400.kHz(),
        &mut pac.RESETS, 
        &clocks.system_clock,
    );

    let mut lsm = lsm6dsox::Lsm6dsox::new(i2c, SlaveAddress::Low, delay);
    lsm.setup().unwrap();
    lsm.set_gyro_sample_rate(DataRate::Freq52Hz).unwrap();
    lsm.set_gyro_scale(GyroscopeScale::Dps500).unwrap();

    let _ =serial.port.write(b"Gyroscope example\r\n");

    loop {
        if serial.device.poll(&mut [&mut serial.port]) {
            if let Ok(data) = lsm.angular_rate() {
                let mut buf: Vec<u8, 512> = Vec::new();

                writeln!(
                    buf, 
                    "gyroscope value x: {:.2}, y: {:.2}, z: {:.2}",
                    data.x.as_hertz(),
                    data.y.as_hertz(),
                    data.z.as_hertz()
                ).unwrap();
    
                let buf=  buf.as_slice();
                let _ = serial.port.write(&buf);
            }
        }
    }
}