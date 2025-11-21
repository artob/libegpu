// This is free and unencumbered software released into the public domain.

use derive_more::Display;
use pyo3::prelude::*;

#[pyclass(frozen, eq, hash, ord, str)]
#[derive(Clone, Debug, Display, Hash, PartialEq, PartialOrd)]
pub struct Enclosure(pub ::egpu::core::Enclosure);

#[pymethods]
impl Enclosure {}
