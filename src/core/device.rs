// This is free and unencumbered software released into the public domain.

use super::Vendor;
use derive_more::Display;

/// PCIe-tunngeled eGPU device.
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{vendor_id}:{device_id}")]
pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) device_id: u16,
    //pub(crate) detail: Option<Box<pci_info::PciDevice>>,
}

impl From<&pci_info::PciDevice> for Device {
    fn from(device: &pci_info::PciDevice) -> Self {
        Device {
            vendor_id: device.vendor_id(),
            device_id: device.device_id(),
            //detail: Some(Box::new(device)),
        }
    }
}

impl Device {
    pub fn vendor(&self) -> Vendor {
        match self.vendor_id() {
            0x1002 => Vendor::Amd,
            0x10DE => Vendor::Nvidia,
            0x8086 => Vendor::Intel,
            _ => Vendor::Other(self.vendor_id()),
        }
    }

    /// The vendor ID of the device.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// The device ID of the device.
    pub fn device_id(&self) -> u16 {
        self.device_id
    }
}
