use pyo3::prelude::*;

pub trait MyTrait: Send + Sync {
    fn len(&self) -> usize;
}

#[pyclass(module = "main_module")]
pub struct MyTraitWrapper {
    inner: Box<dyn MyTrait>,
}

impl MyTraitWrapper {
    pub fn new(m: Box<dyn MyTrait>) -> Self {
        Self { inner: m }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn process_it(m: &MyTraitWrapper) -> PyResult<usize> {
    Ok(m.inner.len())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn main_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_it, m)?)?;
    Ok(())
}
