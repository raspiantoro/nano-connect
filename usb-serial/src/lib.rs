#![no_std]

use core::marker::PhantomData;
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

pub struct InitializedString;
pub struct NoInitializedString;

pub struct VidPid;
pub struct NoVidPid;

pub struct DeviceClass;
pub  struct NoDeviceClass;

pub struct UsbSerialBuilder<'a, B, S, V, D>
where 
    B: UsbBus
{

    alloc: &'a UsbBusAllocator<B>,
    vid_pid: Option<UsbVidPid>,
    device_string: Option<Vec<StringDescriptors<'a>, 16>>,
    device_class: Option<u8>,
    _phantom: (PhantomData<S>, PhantomData<V>, PhantomData<D>)
}

impl<'a, B> UsbSerialBuilder<'a, B, NoInitializedString, NoVidPid, NoDeviceClass> 
where 
    B: UsbBus
{
    pub fn new(alloc: &'a UsbBusAllocator<B>) -> UsbSerialBuilder<'a, B, NoInitializedString, NoVidPid, NoDeviceClass> {
        UsbSerialBuilder{
            alloc: alloc,
            vid_pid: None,
            device_class: None,
            device_string: None,
            _phantom: Default::default()
        }
    }
}

impl<'a, B, V, D> UsbSerialBuilder<'a, B, NoInitializedString, V, D> 
where 
    B: UsbBus
{
    pub fn strings(self, descriptors: &[StringDescriptors<'a>]) -> Result<UsbSerialBuilder<'a, B, InitializedString, V, D>, BuilderError>{
        let device_string = heapless::Vec::from_slice(descriptors).map_err(|_| BuilderError::TooManyLanguages)?;
        Ok(
            UsbSerialBuilder{
                alloc: &self.alloc,
                device_string: Some(device_string),
                vid_pid: self.vid_pid,
                device_class: self.device_class,
                _phantom: Default::default()
            }
        )
    }
}

impl<'a, B, S, D> UsbSerialBuilder<'a, B, S, NoVidPid, D>
where 
    B: UsbBus
{
    pub fn vid_pid(self, vid_pid: UsbVidPid) -> UsbSerialBuilder<'a, B, S, VidPid, D> {
        UsbSerialBuilder{
            alloc: &self.alloc,
            device_class: self.device_class,
            device_string: self.device_string,
            vid_pid: Some(vid_pid),
            _phantom: Default::default()
        }
    }
}

impl<'a, B, S, V> UsbSerialBuilder<'a, B, S, V, NoDeviceClass>
where 
    B: UsbBus
{
    pub fn device_class(self, device_class: u8) -> UsbSerialBuilder<'a, B, S, V, DeviceClass> {
        UsbSerialBuilder{
            alloc: &self.alloc,
            device_class: Some(device_class),
            device_string: self.device_string,
            vid_pid: self.vid_pid,
            _phantom: Default::default()
        }
    }
}

impl<'a, B, S, V, D> UsbSerialBuilder<'a, B, S, V, D> 
where 
    B: UsbBus
{
    pub fn build(self) -> Result<UsbSerial<'a, B>, BuilderError> {
        let descriptor = match self.device_string {
            Some(device_string) => device_string.into_array().unwrap(),
            None => {
                [StringDescriptors::default()
                    .manufacturer("Arduino Nano RP2040")
                    .product("Serial Port")
                    .serial_number("Test")]
            },
        };

        let vid_pid = match self.vid_pid {
            Some(vid_pid) => vid_pid,
            None => UsbVidPid(0, 0),
        };
        
        let device_class = match self.device_class {
            Some(class) => class,

            // set the device class to Communication and CDC Control as the default class
            // learn more about device classes at usb.org/defined-class-codes
            None => 2,
        };

        let device = UsbDeviceBuilder::new(self.alloc, vid_pid)
            .strings(&descriptor)?
            .device_class(device_class);
       
        Ok(UsbSerial{
            port: SerialPort::new(self.alloc),
            device: device.build()
        })
    }   
}