#![no_std]


use heapless::Vec;
use usb_device::{bus::{UsbBus, UsbBusAllocator}, device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid}, prelude::BuilderError};
use usbd_serial::SerialPort;

pub struct UsbSerial<'a, B>
where 
    B: UsbBus
{
    pub port: SerialPort<'a, B>,
    pub device: UsbDevice<'a, B>
}

pub struct UsbSerialBuilder<'a, B>
where 
    B: UsbBus
{

    alloc: &'a UsbBusAllocator<B>,
    vid_pid: UsbVidPid,
    device_string: Vec<StringDescriptors<'a>, 16>
}

impl<'a, B> UsbSerialBuilder<'a, B> 
where 
    B: UsbBus
{
    pub fn new(alloc: &'a UsbBusAllocator<B>, vid_pid: UsbVidPid) -> UsbSerialBuilder<'a, B> {
        UsbSerialBuilder{
            alloc: alloc,
            vid_pid: vid_pid,
            device_string: Vec::new()
        }
    }

    pub fn build(self) -> Result<UsbSerial<'a, B>, BuilderError> {
        let descriptor: [StringDescriptors<'a>; 1];
        
        if self.device_string.len() == 0 {
            descriptor = [StringDescriptors::default()
                .manufacturer("Arduino Nano RP2040")
                .product("Serial Port")
                .serial_number("Test")];
        } else {
            descriptor = self.device_string.into_array().unwrap();
            
        }

        let device = UsbDeviceBuilder::new(self.alloc, self.vid_pid);
       
        Ok(UsbSerial{
            port: SerialPort::new(self.alloc),
            device: device.strings(&descriptor)?.device_class(2).build()
        })
    }

   

    pub fn strings(mut self, descriptors: &[StringDescriptors<'a>]) -> Result<UsbSerialBuilder<'a, B>, BuilderError>{
        self.device_string = heapless::Vec::from_slice(descriptors).map_err(|_| BuilderError::TooManyLanguages)?;
        Ok(self)
    }
}