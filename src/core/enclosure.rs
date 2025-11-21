// This is free and unencumbered software released into the public domain.

use super::Vendor;
use derive_more::Display;

/// USB-attached eGPU enclosure.
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{vendor_id}:{product_id}")]
pub struct Enclosure {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
    //pub(crate) detail: nusb::DeviceInfo,
}

impl From<&nusb::DeviceInfo> for Enclosure {
    fn from(device: &nusb::DeviceInfo) -> Self {
        Enclosure {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
            //detail: Some(Box::new(device)),
        }
    }
}

impl Enclosure {
    pub fn vendor(&self) -> Vendor {
        match self.vendor_id() {
            0x1532 => Vendor::Razer,
            _ => Vendor::Other(self.vendor_id()),
        }
    }

    /// The vendor ID of the enclosure.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// The product ID of the enclosure.
    pub fn product_id(&self) -> u16 {
        self.product_id
    }
}
