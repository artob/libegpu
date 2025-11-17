// This is free and unencumbered software released into the public domain.

use derive_more::Display;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
