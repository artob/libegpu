// This is free and unencumbered software released into the public domain.

mod controller;
mod controllers;
mod device;
mod devices;
mod enclosure;
mod enclosures;

use pyo3::prelude::*;

/// A library for enumerating eGPU devices & enclosures.
#[pymodule]
mod egpu {
    use super::{controllers::Controllers, devices::Devices, enclosures::Enclosures};
    use pyo3::prelude::*;

    #[pymodule_export]
    use crate::{controller::Controller, device::Device, enclosure::Enclosure};

    #[pyfunction]
    pub fn enclosures() -> Enclosures {
        Enclosures
    }

    #[pyfunction]
    pub fn controllers() -> Controllers {
        Controllers
    }

    #[pyfunction]
    pub fn devices() -> Devices {
        Devices
    }
}
