// This is free and unencumbered software released into the public domain.

use derive_more::Display;

/// eGPU bus type.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BusType {
    #[display("PCI")]
    Pci,

    #[display("USB")]
    Usb,
}
