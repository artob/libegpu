// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "usb"))]
#[derive(Clone, Debug)]
pub struct Enclosure;

#[cfg(feature = "usb")]
#[derive(Clone, Debug)]
pub struct Enclosure(pub nusb::DeviceInfo);
