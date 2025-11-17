// This is free and unencumbered software released into the public domain.

use super::Vendor;

#[cfg(not(feature = "usb"))]
#[derive(Clone, Debug)]
pub struct Enclosure;

/// USB-attached eGPU enclosure.
#[cfg(feature = "usb")]
#[derive(Clone, Debug)]
pub struct Enclosure(pub nusb::DeviceInfo);

#[cfg(feature = "usb")]
impl Enclosure {
    pub fn vendor(&self) -> Vendor {
        match self.vendor_id() {
            0x1532 => Vendor::Razer,
            _ => Vendor::Other(self.vendor_id()),
        }
    }

    /// The vendor ID of the enclosure.
    pub fn vendor_id(&self) -> u16 {
        self.0.vendor_id()
    }

    /// The product ID of the enclosure.
    pub fn product_id(&self) -> u16 {
        self.0.product_id()
    }
}
