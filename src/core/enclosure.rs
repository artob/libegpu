// This is free and unencumbered software released into the public domain.

use nusb::DeviceInfo;

#[derive(Clone, Debug)]
pub struct Enclosure(pub DeviceInfo);
