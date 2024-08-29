#![no_std]
#![no_main]

use panic_halt as _;
use arduino_nano_connect as bsp;
use bsp::pac;
use bsp::hal;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::UsbVidPid;

#[arduino_nano_connect::entry]
fn main () -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

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

    let sio = hal::Sio::new(pac.SIO);

    let _pins = hal::gpio::Pins::new(
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
    
    serial.port.write(b"USB Serial Echo Server\r\n").unwrap();

    loop {
        if serial.device.poll(&mut [&mut serial.port]){
            let mut buf = [0u8; 64];

            match serial.port.read(&mut buf) {
                Ok(0) => {},
                Ok(size) => {
                    let mut wr_ptr = &buf[..size];
                    while !wr_ptr.is_empty() {
                        serial.port.write(b"reply with: ").unwrap();

                        match serial.port.write(wr_ptr) {
                            Ok(len) => wr_ptr = &wr_ptr[len..],
                            Err(_) => break,
                        }

                        serial.port.write(b"\r\n").unwrap();
                    }
                }
                Err(_) => {},
            }   
        }
    }
}
