// This is free and unencumbered software released into the public domain.

/// Core types and traits.
pub mod core {
    mod bus;
    pub use bus::*;

    #[cfg(feature = "pci")]
    mod device;
    #[cfg(feature = "pci")]
    pub use device::*;

    #[cfg(feature = "usb")]
    mod enclosure;
    #[cfg(feature = "usb")]
    pub use enclosure::*;

    mod vendor;
    pub use vendor::*;
}

/// PCIe device enumeration support.
#[cfg(feature = "pci")]
pub mod pci {
    mod controllers;
    pub use controllers::*;
    mod devices;
    pub use devices::*;
    mod iterator;
    pub use iterator::*;
    pub mod registry;
}

/// USB enclosure enumeration support.
#[cfg(feature = "usb")]
pub mod usb {
    mod enclosures;
    pub use enclosures::*;
    pub mod registry;
}

#[cfg(feature = "pci")]
pub use pci::{list_controllers, list_devices};

#[cfg(feature = "usb")]
pub use usb::list_enclosures;
