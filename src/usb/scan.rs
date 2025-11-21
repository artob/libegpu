// This is free and unencumbered software released into the public domain.

use crate::{
    core::Enclosure,
    usb::registry::{KNOWN_ENCLOSURES, KNOWN_VENDORS},
};
use nusb::{DeviceInfo, Error, MaybeFuture};

struct EnclosureIterator(Box<dyn Iterator<Item = DeviceInfo>>);

/// Enumerate USB-attached eGPU enclosures.
pub fn list_enclosures() -> Result<impl Iterator<Item = Enclosure>, Error> {
    Ok(EnclosureIterator(Box::new(nusb::list_devices().wait()?)))
}

impl Iterator for EnclosureIterator {
    type Item = Enclosure;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(device) = self.0.next() {
            if KNOWN_VENDORS
                .iter()
                .find(|&vid| *vid == device.vendor_id())
                .is_some()
                && let Some(_dev) = KNOWN_ENCLOSURES
                    .iter()
                    .find(|&dev| dev.vid == device.vendor_id() && dev.pid == device.product_id())
            {
                return Some(Enclosure::from(&device));
            }
        }
        None
    }
}
