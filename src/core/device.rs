// This is free and unencumbered software released into the public domain.

use pci_info::PciDevice;

#[derive(Debug)]
pub struct Device(pub PciDevice);
