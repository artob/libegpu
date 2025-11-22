// This is free and unencumbered software released into the public domain.

use crate::{core::Device, pci::registry::KNOWN_VENDORS};
use pci_info::{PciDevice, PciDeviceEnumerationError, pci_enums::PciDeviceClass};

pub struct DeviceIterator {
    pub filter_class: PciDeviceClass,
    pub inner: Box<dyn Iterator<Item = Result<PciDevice, PciDeviceEnumerationError>>>,
}

impl Iterator for DeviceIterator {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.inner.next() {
            match result {
                Err(_) => continue, // skip erroneous devices
                Ok(device) => {
                    let Ok(device_class) = device.device_class() else {
                        continue; // skip erroneous devices
                    };
                    if device_class == self.filter_class
                        && KNOWN_VENDORS
                            .iter()
                            .find(|&vid| *vid == device.vendor_id())
                            .is_some()
                    {
                        return Some(Device::from(&device));
                    }
                },
            }
        }
        None
    }
}
