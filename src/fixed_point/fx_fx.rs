use super::FixedPoint;
use crate::{fixed_point::debug_print, mask, to_float, UInt};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Fx {
  pub val: UInt,
  pub m: i32,
  pub b: i32,
  pub is_exact: bool,
}

impl Fx {
  pub fn new(val: UInt, m: i32, b: i32, is_exact: bool) -> Self {
    if b < m {
      panic!("Total num of bits must be larger than num of integer bits.")
    }
    Self {
      val,
      m,
      b,
      is_exact,
    }
  }

  pub fn get_frac_bits(&self) -> i32 {
    self.b - self.m
  }
}

impl FixedPoint for Fx {
  fn eval(&self) -> f64 {
    to_float(self.val, self.b, self.m, self.b - self.m - 1).unwrap()
  }
}

impl std::ops::Shl<u32> for Fx {
  type Output = Self;
  fn shl(self, rhs: u32) -> Self::Output {
    Self {
      val: (self.val << rhs) & mask((self.b) as u32) as UInt,
      m: self.m,
      b: self.b,
      is_exact: self.is_exact,
    }
  }
}

impl std::ops::Shr<u32> for Fx {
  type Output = Self;
  fn shr(self, rhs: u32) -> Self::Output {
    // let val = (self.val >> rhs) & mask((self.b) as u32) as u64;
    let val = match self.val.checked_shr(rhs) {
      Some(v) => v & mask((self.b) as u32) as UInt,
      None => 0,
    };

    Self {
      val,
      m: self.m,
      b: self.b,
      is_exact: self.is_exact,
    }
  }
}

impl std::ops::Add for Fx {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    if self.m != rhs.m || self.b != rhs.b {
      // panic!("`m` and `n` field of each fx obj has to match.")
      return add_incoherent(self, rhs);
    }
    Self {
      val: self.val + rhs.val,
      m: self.m,
      b: self.b,
      is_exact: self.is_exact && rhs.is_exact,
    }
  }
}

impl std::ops::Sub for Fx {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    if rhs.eval() > self.eval() {
      unimplemented!()
    }
    Self {
      val: self.val - rhs.val,
      m: self.m,
      b: self.b,
      is_exact: true,
    }
  }
}

impl std::ops::Mul for Fx {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      val: self.val * rhs.val,
      m: self.m + rhs.m,
      b: self.b + rhs.b,
      is_exact: true,
    }
  }
}

impl std::fmt::Debug for Fx {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ans = debug_print(self.val, self.m, self.b, self.is_exact);
    write!(f, "{}", ans)
  }
}

impl std::fmt::Display for Fx {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ans;
    ans = format!("Fx<{},{}>({})", self.m, self.b, self.val);
    write!(f, "{}", ans)
  }
}

#[allow(non_snake_case)]
pub fn to_Fx(x: f64, m: i32, b: i32, round: bool) -> Result<Fx, String> {
  let fx_q = crate::to_fixed(x, m, b - m - 1, round);
  match fx_q {
    Ok(fx) => Ok(Fx::new(fx.val, fx.m, fx.m + fx.n + 1, fx.is_exact)),
    Err(e) => Err(e.to_string()),
  }
}

/// Addition of Fx types belonging to different families
/// e.g. Fx<5, 10> + Fx<2, 40> => Fx<5, 40> + Fx<5, 40> => Fx<6, 41>
pub fn add_incoherent(fx1: Fx, fx2: Fx) -> Fx {
  // println!("{:?}", fx1);
  // println!("{:?}", fx2);

  let (m_new, b_new) = (std::cmp::max(fx1.m, fx2.m), std::cmp::max(fx1.b, fx2.b));
  /*
  let fx1_new = Fx::new(fx1.val << (b_new - fx1.b), m_new, b_new, false);
  let fx2_new = Fx::new(fx2.val >> (m_new - fx2.m), m_new, b_new, false);
  */
  let fx1_new = to_Fx(fx1.eval(), m_new, b_new, true).unwrap();
  let fx2_new = to_Fx(fx2.eval(), m_new, b_new, true).unwrap();

  // println!("{:?}", fx1_new);
  // println!("{:?}", fx2_new);

  fx1_new + fx2_new
}

#[cfg(test)]
mod test {

  use crate::fixed_point::{to_Fx, Fx};

  #[test]
  fn test_add_incoherent_1() {
    let fx1 = to_Fx(10.2, 5, 10, true).unwrap();
    let fx2 = to_Fx(2.2, 2, 40, true).unwrap();
    assert_eq!(
      fx1 + fx2,
      Fx::new(0b00110001100011001100110011001100110011010, 6, 41, true)
    );
  }

  #[test]
  fn test_add_incoherent_2() {
    let fx1 = to_Fx(1.1, 6, 10, true).unwrap();
    let fx2 = to_Fx(2.2, 4, 20, true).unwrap();
    assert_eq!(fx1 + fx2, Fx::new(0b000001101010011001101, 7, 21, true));
  }
}
