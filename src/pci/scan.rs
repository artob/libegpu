// This is free and unencumbered software released into the public domain.

use crate::{core::Device, pci::registry::KNOWN_VENDORS};
use pci_info::{PciDevice, PciDeviceEnumerationError, PciInfo, PciInfoError};

struct DeviceIterator(Box<dyn Iterator<Item = Result<PciDevice, PciDeviceEnumerationError>>>);

pub fn list_devices() -> Result<impl Iterator<Item = Device>, PciInfoError> {
    let info = PciInfo::enumerate_pci()?;
    Ok(DeviceIterator(Box::new(info.into_iter())))
}

impl Iterator for DeviceIterator {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.0.next() {
            match result {
                Err(_) => continue, // skip errors
                Ok(device) => {
                    if KNOWN_VENDORS
                        .iter()
                        .find(|&vid| *vid == device.vendor_id())
                        .is_some()
                    {
                        return Some(Device(device));
                    }
                },
            }
        }
        None
    }
}
