// This is free and unencumbered software released into the public domain.

use derive_more::Display;

/// eGPU enclosure and/or device vendor.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Vendor {
    #[display("AMD")]
    Amd,

    #[display("Intel")]
    Intel,

    #[display("NVIDIA")]
    Nvidia,

    #[display("Razer")]
    Razer,

    #[display("Other (_0)")]
    Other(u16),
}
