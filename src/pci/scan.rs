// This is free and unencumbered software released into the public domain.

use crate::{core::Device, pci::registry::KNOWN_VENDORS};
use pci_info::{
    PciDevice, PciDeviceEnumerationError, PciInfo, PciInfoError, pci_enums::PciDeviceClass,
};

struct DeviceIterator(Box<dyn Iterator<Item = Result<PciDevice, PciDeviceEnumerationError>>>);

/// Enumerate PCIe-tunneled (over USB4/Thunderbolt) eGPU devices.
pub fn list_devices() -> Result<impl Iterator<Item = Device>, PciInfoError> {
    let info = PciInfo::enumerate_pci()?;
    Ok(DeviceIterator(Box::new(info.into_iter())))
}

impl Iterator for DeviceIterator {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.0.next() {
            match result {
                Err(_) => continue, // skip erroneous devices
                Ok(device) => {
                    let Ok(device_class) = device.device_class() else {
                        continue; // skip erroneous devices
                    };
                    if device_class == PciDeviceClass::DisplayController
                        && KNOWN_VENDORS
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
