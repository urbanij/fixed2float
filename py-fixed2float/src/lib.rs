use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

extern crate fixed2float;
use fixed2float as f2f;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyfunction]
fn to_fixed(x: f64, m: u8, n: u8) -> PyResult<u64> {
    Ok(f2f::to_fixed(x, m, n).unwrap().0)
}

#[pyfunction]
fn to_float(bits: u64, m: u8, n: u8) -> PyResult<f64> {
    Ok(f2f::to_float(bits, m, n))
}
/// A Python module implemented in Rust.
#[pymodule]
fn fixed2float(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_fixed, m)?)?;
    m.add_function(wrap_pyfunction!(to_float, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
