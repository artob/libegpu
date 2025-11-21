// This is free and unencumbered software released into the public domain.

mod bus;
mod device;
mod enclosure;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod egpu {
    #[pymodule_export]
    use crate::{bus::Bus, device::Device, enclosure::Enclosure};
}
