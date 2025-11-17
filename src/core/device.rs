// This is free and unencumbered software released into the public domain.

use super::Vendor;

#[cfg(not(feature = "pci"))]
#[derive(Debug)]
pub struct Device;

#[cfg(feature = "pci")]
#[derive(Debug)]
pub struct Device(pub pci_info::PciDevice);

#[cfg(feature = "pci")]
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
        self.0.vendor_id()
    }

    /// The device ID of the device.
    pub fn device_id(&self) -> u16 {
        self.0.device_id()
    }
}
