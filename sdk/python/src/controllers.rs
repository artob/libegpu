// This is free and unencumbered software released into the public domain.

use crate::device::Device;
use derive_more::Display;
use pyo3::{exceptions::PySystemError, prelude::*};

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct Controllers;

#[pymethods]
impl Controllers {
    fn __iter__(this: PyRef<'_, Self>) -> PyResult<Py<DeviceIterator>> {
        let devices: Vec<::egpu::core::Device> = ::egpu::list_controllers()
            .map_err(|err| PySystemError::new_err(err.to_string()))?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Py::new(this.py(), DeviceIterator(devices.into_iter()))?)
    }
}

#[pyclass]
pub struct DeviceIterator(std::vec::IntoIter<::egpu::core::Device>);

#[pymethods]
impl DeviceIterator {
    fn __iter__(this: PyRef<'_, Self>) -> PyRef<'_, Self> {
        this
    }

    fn __next__(mut this: PyRefMut<'_, Self>) -> Option<Device> {
        this.0.next().map(|device| Device(device))
    }
}
