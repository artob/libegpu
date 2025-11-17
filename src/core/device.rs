// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "pci"))]
#[derive(Clone, Debug)]
pub struct Device;

#[cfg(feature = "pci")]
#[derive(Debug)]
pub struct Device(pub pci_info::PciDevice);
