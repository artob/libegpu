// This is free and unencumbered software released into the public domain.

use super::DeviceIterator;
use crate::core::Device;
use pci_info::{PciInfo, PciInfoError, pci_enums::PciDeviceClass};

/// Enumerate PCIe-tunneled (over USB4/Thunderbolt) eGPU devices.
pub fn list_devices() -> Result<impl Iterator<Item = Device>, PciInfoError> {
    let info = PciInfo::enumerate_pci()?;
    Ok(DeviceIterator {
        filter_class: PciDeviceClass::DisplayController,
        inner: Box::new(info.into_iter()),
    })
}
