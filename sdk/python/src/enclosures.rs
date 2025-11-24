// This is free and unencumbered software released into the public domain.

use crate::enclosure::Enclosure;
use derive_more::Display;
use pyo3::{exceptions::PySystemError, prelude::*};

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct Enclosures;

#[pymethods]
impl Enclosures {
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
