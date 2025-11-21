// This is free and unencumbered software released into the public domain.

use super::{device::Device, enclosure::Enclosure};
use derive_more::Display;
use pyo3::{exceptions::PySystemError, prelude::*};

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct Bus;

#[pymethods]
impl Bus {
    #[new]
    fn new() -> Bus {
        Bus
    }

    fn devices(&self) -> BusDevices {
        BusDevices
    }

    fn enclosures(&self) -> BusDevices {
        BusDevices
    }
}

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct BusDevices;

#[pymethods]
impl BusDevices {
    fn __iter__(this: PyRef<'_, Self>) -> PyResult<Py<DeviceIterator>> {
        let devices: Vec<::egpu::core::Device> = ::egpu::list_devices()
            .map_err(|err| PySystemError::new_err(err.to_string()))?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Py::new(this.py(), DeviceIterator(devices.into_iter()))?)
    }
}

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct BusEnclosures;

#[pymethods]
impl BusEnclosures {
    fn __iter__(this: PyRef<'_, Self>) -> PyResult<Py<EnclosureIterator>> {
        let enclosures: Vec<::egpu::core::Enclosure> = ::egpu::list_enclosures()
            .map_err(|err| PySystemError::new_err(err.to_string()))?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Py::new(
            this.py(),
            EnclosureIterator(enclosures.into_iter()),
        )?)
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

#[pyclass]
pub struct EnclosureIterator(std::vec::IntoIter<::egpu::core::Enclosure>);

#[pymethods]
impl EnclosureIterator {
    fn __iter__(this: PyRef<'_, Self>) -> PyRef<'_, Self> {
        this
    }

    fn __next__(mut this: PyRefMut<'_, Self>) -> Option<Enclosure> {
        this.0.next().map(|enclosure| Enclosure(enclosure))
    }
}
