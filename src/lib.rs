// This is free and unencumbered software released into the public domain.

pub mod core {
    mod device;
    pub use device::*;
    mod enclosure;
    pub use enclosure::*;
    mod vendor;
    pub use vendor::*;
}

#[cfg(feature = "pci")]
pub mod pci {
    mod scan;
    pub use scan::*;
    pub mod registry;
}

#[cfg(feature = "usb")]
pub mod usb {
    mod scan;
    pub use scan::*;
    pub mod registry;
}
