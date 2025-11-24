// This is free and unencumbered software released into the public domain.

use super::{BusType, Vendor};
use derive_more::Display;

/// USB-attached eGPU enclosure.
#[derive(Clone, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("{vendor_id}:{product_id}")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Enclosure {
    pub(crate) path: Option<String>,
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
    pub(crate) product_string: Option<String>,
    //pub(crate) detail: nusb::DeviceInfo,
}

impl core::fmt::Debug for Enclosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enclosure")
            .field("path", &self.path())
            .field("bus_type", &self.bus_type())
            .field("vendor", &self.vendor())
            .field("vendor_id", &self.vendor_id)
            .field("product_id", &self.product_id)
            .field("product_string", &self.product_id)
            .finish()
    }
}

impl From<&nusb::DeviceInfo> for Enclosure {
    fn from(device: &nusb::DeviceInfo) -> Self {
        Enclosure {
            #[cfg(target_os = "linux")]
            path: Some(device.sysfs_path().display().to_string()),
            #[cfg(not(target_os = "linux"))]
            path: None,
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
            product_string: device.product_string().map(String::from),
            //detail: Some(Box::new(device)),
        }
    }
}

impl Enclosure {
    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    pub fn bus_type(&self) -> BusType {
        BusType::Usb
    }

    pub fn vendor(&self) -> Vendor {
        Vendor::from_usb_vid(self.vendor_id())
    }

    /// The vendor ID of the enclosure.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// The product ID of the enclosure.
    pub fn product_id(&self) -> u16 {
        self.product_id
    }

    /// The product string of the enclosure, if available.
    pub fn product_string(&self) -> Option<&str> {
        self.product_string.as_deref()
    }
}
