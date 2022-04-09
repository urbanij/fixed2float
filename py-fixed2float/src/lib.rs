use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use f2f::FixedPoint;
use fixed2float as f2f;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyclass(name = "Fx")]
// #[repr(transparent)]
#[derive(Clone)]
pub struct Fx {
    #[pyo3(get)]
    pub val: u128,
    #[pyo3(get)]
    pub m: i32,
    #[pyo3(get)]
    pub b: i32,
    #[pyo3(get)]
    pub is_exact: bool,
    pub fx: f2f::Fx,
}

impl From<f2f::Fx> for Fx {
    fn from(fx: f2f::Fx) -> Self {
        Self {
            val: fx.val,
            m: fx.m,
            b: fx.b,
            is_exact: fx.is_exact,
            fx,
        }
    }
}

#[pymethods]
impl Fx {
    #[new]
    fn new(val: u128, m: i32, b: i32) -> Self {
        Self {
            val,
            m,
            b,
            is_exact: true,
            fx: f2f::Fx::new(val, m, b, true),
        }
    }

    pub fn get_val(&self) -> u128 {
        self.fx.val
    }

    pub fn get_frac_bits(&self) -> i32 {
        self.fx.b - self.fx.m
    }

    pub fn eval(&self) -> f64 {
        self.fx.eval()
    }

    fn __add__(&self, other: Self) -> Self {
        (self.fx + other.fx).into()
    }

    fn __sub__(&self, other: Self) -> Self {
        (self.fx - other.fx).into()
    }

    fn __mul__(&self, other: Self) -> Self {
        (self.fx * other.fx).into()
    }

    fn __lshift__(&self, other: i32) -> Self {
        (self.fx << other as u32).into()
    }

    fn __rshift__(&self, other: i32) -> Self {
        (self.fx >> other as u32).into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.fx)
    }

    pub fn as_str(&self) -> String {
        format!("{}", self.fx)
    }
}

#[pyfunction]
#[allow(non_snake_case)]
fn to_Fx(x: f64, m: i32, b: i32, round: bool) -> PyResult<Option<Fx>> {
    let ans = f2f::to_Fx(x, m, b, round);
    match ans {
        Ok(fx) => Ok(Some(Fx::from(fx))),
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
    m.add_class::<Fx>()?;
    m.add_function(wrap_pyfunction!(to_Fx, m)?)?;
    m.add_function(wrap_pyfunction!(to_float, m)?)?;
    m.add_function(wrap_pyfunction!(to_float_str, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
