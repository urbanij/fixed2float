#![allow(non_snake_case)]

use pyo3::prelude::*;

use ::fixed2float::FixedPoint;
use ::fixed2float::{to_Fx, Fx};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
  VERSION
}

#[pyclass]
#[derive(Clone, Debug)]
struct PyFx {
  inner: Fx,
}

#[pymethods]
impl PyFx {
  #[new]
  fn new(val: u128, m: i32, b: i32) -> Self {
    Self {
      inner: Fx {
        val,
        m,
        b,
        is_exact: true,
      },
    }
  }

  #[getter]
  fn get_val(&self) -> u128 {
    // i can access the val attribute just with `obj.val`
    self.inner.val
  }

  pub fn get_sign(&self) -> i32 {
    (self.inner.val >> (self.inner.b)).try_into().unwrap()
  }

  pub fn get_int_bits(&self) -> i32 {
    self.inner.m
  }

  pub fn get_frac_bits(&self) -> i32 {
    self.inner.b - self.inner.m
  }

  pub fn eval(&self) -> f64 {
    self.inner.eval()
  }

  fn __add__(&self, other: Self) -> Self {
    Self {
      inner: self.inner + other.inner,
    }
  }

  fn __neg__(&self) -> Self {
    Self { inner: -self.inner }
  }

  fn __sub__(&self, other: Self) -> Self {
    Self {
      inner: self.inner + (-other.inner),
    }
  }

  // fn __mul__(&self, other: Self) -> Self {
  //   Self {
  //     inner: self.inner * other.inner,
  //   }
  // }

  fn __repr__(&self) -> PyResult<String> {
    let ans = format!("{:?} {} {}", self.inner, self.inner, self.eval());
    Ok(ans)
  }
}

#[pyfunction]
#[deprecated(since = "0.4.0")]
#[pyo3(signature = (x, m, b, round=false))]
fn py_to_Fx(x: f64, m: i32, b: i32, round: bool) -> PyResult<Option<PyFx>> {
  py_from_double(x, m, b, round)
}

#[pyfunction]
#[pyo3(signature = (x, m, b, round=false))]
fn py_from_double(x: f64, m: i32, b: i32, round: bool) -> PyResult<Option<PyFx>> {
  let ans = to_Fx(x, m, b, round);
  match ans {
    Ok(fx) => Ok(Some(PyFx { inner: fx })),
    Err(e) => {
      println!("{}", e);
      Ok(None)
    }
  }
}

#[pyfunction]
fn py_from_bits(val: u128, m: i32, b: i32) -> PyResult<Option<PyFx>> {
  let fx = Fx::new(val, m, b, true);
  Ok(Some(PyFx { inner: fx }))
}

// #[pyfunction]
// fn to_float(bits: u128, size: i32, m: i32, n: i32) -> PyResult<Option<f64>> {
//   let ans = f2f::to_float(bits, size, m, n);
//   match ans {
//     Ok(x) => Ok(Some(x)),
//     Err(e) => {
//       println!("{}", e);
//       Ok(None)
//     }
//   }
// }

// #[pyfunction]
// fn to_float_str(bits: &str, m: i32, n: i32) -> PyResult<Option<f64>> {
//   let ans = f2f::to_float_str(bits, m, n);
//   match ans {
//     Ok(x) => Ok(Some(x)),
//     Err(e) => {
//       println!("{}", e);
//       Ok(None)
//     }
//   }
// }

/// A Python module implemented in Rust.
#[pymodule]
fn fixed2float(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<PyFx>()?;
  m.add_function(wrap_pyfunction!(py_from_double, m)?)?;
  m.add_function(wrap_pyfunction!(py_from_bits, m)?)?;
  m.add_function(wrap_pyfunction!(py_to_Fx, m)?)?;
  m.add_function(wrap_pyfunction!(version, m)?)?;
  Ok(())
}
