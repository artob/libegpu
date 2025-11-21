// This is free and unencumbered software released into the public domain.

/// Core types and traits.
pub mod core {
    #[cfg(feature = "pci")]
    mod device;
    #[cfg(feature = "pci")]
    pub use device::*;
    #[cfg(feature = "pci")]
    mod enclosure;
    #[cfg(feature = "pci")]
    pub use enclosure::*;
    mod vendor;
    pub use vendor::*;
}

/// PCIe device enumeration support.
#[cfg(feature = "pci")]
pub mod pci {
    mod scan;
    pub use scan::*;
    pub mod registry;
}

/// USB enclosure enumeration support.
#[cfg(feature = "usb")]
pub mod usb {
    mod scan;
    pub use scan::*;
    pub mod registry;
}

#[cfg(feature = "pci")]
pub use pci::list_devices;

#[cfg(feature = "usb")]
pub use usb::list_enclosures;
