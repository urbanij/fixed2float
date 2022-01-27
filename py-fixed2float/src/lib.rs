use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use fixed2float as f2f;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyclass(name = "FixedPoint")]
// #[repr(transparent)]
#[derive(Clone)]
pub struct FixedPoint {
    #[pyo3(get)]
    pub val: u128,
    #[pyo3(get)]
    pub m: i32,
    #[pyo3(get)]
    pub n: i32,
    #[pyo3(get)]
    pub is_exact: bool,
    pub fp: f2f::FixedPoint,
}

impl From<f2f::FixedPoint> for FixedPoint {
    fn from(fp: f2f::FixedPoint) -> Self {
        Self {
            val: fp.val,
            m: fp.m,
            n: fp.n,
            is_exact: fp.is_exact,
            fp,
        }
    }
}

#[pymethods]
impl FixedPoint {
    #[new]
    fn new(val: u128, m: i32, n: i32) -> Self {
        Self {
            val,
            m,
            n,
            is_exact: true,
            fp: f2f::FixedPoint::new(val, m, n, true),
        }
    }

    pub fn get_val(&self) -> u128 {
        self.fp.val
    }

    pub fn eval(&self) -> f64 {
        self.fp.eval()
    }

    fn __add__(&self, other: Self) -> Self {
        (self.fp + other.fp).into()
    }

    fn __sub__(&self, other: Self) -> Self {
        (self.fp - other.fp).into()
    }

    fn __mul__(&self, other: Self) -> Self {
        (self.fp * other.fp).into()
    }

    fn __lshift__(&self, other: i32) -> Self {
        (self.fp << other as u32).into()
    }

    fn __rshift__(&self, other: i32) -> Self {
        (self.fp >> other as u32).into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.fp)
    }
}

// impl std::ops::Add for FixedPoint {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {

//     }
// }

#[pyfunction]
fn to_fixed(x: f64, m: i32, n: i32) -> PyResult<Option<FixedPoint>> {
    let ans = f2f::to_fixed(x, m, n);
    match ans {
        Ok(fp) => Ok(Some(FixedPoint::from(fp))),
        Err(e) => {
            println!("{}", e);
            Ok(None)
        }
    }
}

#[pyfunction]
fn to_float(bits: u128, size: i32, m: i32, n: i32) -> PyResult<Option<f64>> {
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
    m.add_class::<FixedPoint>()?;
    m.add_function(wrap_pyfunction!(to_fixed, m)?)?;
    m.add_function(wrap_pyfunction!(to_float, m)?)?;
    m.add_function(wrap_pyfunction!(to_float_str, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
