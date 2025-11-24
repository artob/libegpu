// This is free and unencumbered software released into the public domain.

use super::BusType;
use derive_more::Display;

/// eGPU enclosure and/or device vendor.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Vendor {
    #[display("AMD")]
    Amd,

    #[display("ASMedia")]
    AsMedia,

    #[display("Intel")]
    Intel,

    #[display("NVIDIA")]
    Nvidia,

    #[display("Razer")]
    Razer,

    #[display("Other (_0)")]
    Other(BusType, u16),
}

impl Vendor {
    pub fn from_pci_vid(vid: u16) -> Self {
        match vid {
            0x1002 => Self::Amd,
            0x10DE => Self::Nvidia,
            0x1B21 => Self::AsMedia,
            0x8086 => Self::Intel,
            _ => Self::Other(BusType::Pci, vid),
        }
    }

    pub fn from_usb_vid(vid: u16) -> Self {
        match vid {
            0x1532 => Self::Razer,
            0x174C => Self::AsMedia,
            0x8086 | 0x8087 => Self::Intel,
            _ => Self::Other(BusType::Usb, vid),
        }
    }
}
