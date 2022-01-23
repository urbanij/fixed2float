use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use fixed2float as f2f;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyfunction]
fn to_fixed(x: f64, m: i32, n: i32) -> PyResult<Option<(u128, bool)>> {
    let ans = f2f::to_fixed(x, m, n);
    match ans {
        Ok((bits, is_exact)) => Ok(Some((bits, is_exact))),
        Err(e) => {
            println!("{}", e);
            Ok(None)
        }
    }
}

#[pyfunction]
fn to_float(bits: i64, size: i32, m: i32, n: i32) -> PyResult<Option<f64>> {
    let ans = f2f::to_float(bits, size, m, n);
    match ans {
        Ok(x) => Ok(Some(x)),
        Err(e) => {
            println!("{}", e);
            Ok(None)
        }
    }
}

#[pyfunction]
fn to_float_str(bits: &str, m: i32, n: i32) -> PyResult<Option<f64>> {
    let ans = f2f::to_float_str(bits, m, n);
    match ans {
        Ok(x) => Ok(Some(x)),
        Err(e) => {
            println!("{}", e);
            Ok(None)
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn fixed2float(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_fixed, m)?)?;
    m.add_function(wrap_pyfunction!(to_float, m)?)?;
    m.add_function(wrap_pyfunction!(to_float_str, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
