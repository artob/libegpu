// This is free and unencumbered software released into the public domain.

use super::{BusType, Vendor};
use derive_more::Display;

/// PCIe-tunneled eGPU device.
#[derive(Clone, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{vendor_id}:{device_id}")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Device {
    pub(crate) path: Option<String>,
    pub(crate) vendor_id: u16,
    pub(crate) device_id: u16,
    //pub(crate) detail: Option<Box<pci_info::PciDevice>>,
}

impl core::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("path", &self.path())
            .field("bus_type", &self.bus_type())
            .field("vendor", &self.vendor())
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("removable", &self.is_removable())
            .finish()
    }
}

impl From<&pci_info::PciDevice> for Device {
    fn from(device: &pci_info::PciDevice) -> Self {
        Device {
            path: if let Ok(location) = device.location() {
                if cfg!(target_os = "linux") {
                    // See: https://docs.kernel.org/PCI/sysfs-pci.html
                    Some(format!("/sys/bus/pci/devices/{}", location))
                } else {
                    None // TODO: support other platforms as well
                }
            } else {
                None
            },
            vendor_id: device.vendor_id(),
            device_id: device.device_id(),
            //detail: Some(Box::new(device)),
        }
    }
}

impl Device {
    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    pub fn bus_type(&self) -> BusType {
        BusType::Pci
    }

    pub fn vendor(&self) -> Vendor {
        Vendor::from_pci_vid(self.vendor_id())
    }

    /// The vendor ID of the device.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// The device ID of the device.
    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    pub fn is_removable(&self) -> Option<bool> {
        #[cfg(feature = "std")]
        if cfg!(target_os = "linux") {
            return self
                .path()
                .as_ref()
                .map(|path| std::fs::metadata(path).is_ok());
        }
        None // unknown
    }
}
