use super::FixedPoint;
use crate::{fixed_point::debug_print, mask, to_fixed, to_float, UInt};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Fx {
  pub val: UInt,
  pub m: i32,
  pub b: i32,
  pub is_exact: bool,
}

impl Fx {
  pub fn new(val: UInt, m: i32, b: i32, is_exact: bool) -> Self {
    if b < 1 + m {
      panic!("Total num of bits must be larger than num of integer bits + sign.")
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

impl std::ops::Add for Fx {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    if self.m != rhs.m || self.b != rhs.b {
      panic!("`m` and `n` field of each fx obj has to match.")
      // return add_incoherent(self, rhs);
    }

    let (m, b) = (self.m, self.b);

    let sum_eval = self.eval() + rhs.eval();
    if sum_eval.abs().log2() >= m as f64 {
      panic!("{} can't fit into {} integer bits", sum_eval, m);
    }

    let (fixed1_val, fixed2_val) = (self.val, rhs.val);

    let fixed_sum = (fixed1_val + fixed2_val) & mask(b as u32);

    // handling special case: a + (-a) = 0
    if (fixed_sum) & mask(b as u32 - 1) == 0 {
      return Self {
        val: 0,
        m,
        b,
        is_exact: true,
      };
    }

    let fixed_sum_sign = (fixed_sum >> (b - 1)) as i32;

    let fixed_sum_abs = match fixed_sum_sign == 0 {
      true => fixed_sum,
      false => (!fixed_sum + 1) & mask(b as u32),
    };

    let new_val = if (fixed_sum_abs as f32).log2() < (self.b - 1) as f32 {
      fixed_sum
    } else {
      panic!(
        "Can't fit {} into `m` = {} integer bits",
        fixed_sum_abs, self.m
      )
    };

    Self {
      val: new_val,
      m: self.m,
      b: self.b,
      is_exact: self.is_exact && rhs.is_exact,
    }
  }
}

impl std::ops::Sub for Fx {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    self + (-rhs)
  }
}

impl std::ops::Neg for Fx {
  type Output = Self;
  fn neg(self) -> Self::Output {
    if self.val == 0 {
      return self;
    }
    Self {
      val: (!self.val + 1) & mask(self.b as u32),
      m: self.m,
      b: self.b,
      is_exact: self.is_exact,
    }
  }
}

impl std::ops::Mul for Fx {
  type Output = Self;
  ///
  /// ```rust
  /// use fixed2float::Fx;
  /// let m = Fx::new(0b0_0001_100, 4, 8, true) * Fx::new(0b1_1101_010, 4, 8, true); // 1.5 * -2.75 == -4.125
  /// let mul = Fx::new(0b1_1011_111, 4, 8, true);
  /// assert_eq!(m, mul);
  /// ```
  fn mul(self, rhs: Self) -> Self::Output {
    if self.m != rhs.m || self.b != rhs.b {
      panic!("`m` and `n` field of each fx obj has to match.")
      // return add_incoherent(self, rhs);
    }

    let (m, b) = (self.m, self.b);
    let n = b - m - 1;

    let mul_eval = self.eval() * rhs.eval();

    if mul_eval.abs().log2() >= m as f64 {
      panic!("{} can't fit into {} integer bits", mul_eval, m);
    }

    let sign_val1 = self.val >> (b - 1);
    let sign_val2 = rhs.val >> (b - 1);

    let abs_val1 = match sign_val1 == 0 {
      true => self.val,
      false => (!self.val + 1) & mask(b as u32),
    };

    let abs_val2 = match sign_val2 == 0 {
      true => rhs.val,
      false => (!rhs.val + 1) & mask(b as u32),
    };

    let abs_val_mul = abs_val1 * abs_val2;
    let fixed_mul_sign = sign_val1 ^ sign_val2;

    let is_exact = (self.is_exact && rhs.is_exact) && (abs_val_mul & mask(n as u32) == 0);

    let abs_val_mul_adjusted = abs_val_mul >> n;

    let val_mul_adjusted = match fixed_mul_sign == 0 {
      true => abs_val_mul_adjusted,
      false => (!abs_val_mul_adjusted + 1) & mask(b as u32),
    };

    Self {
      val: val_mul_adjusted,
      m: self.m,
      b: self.b,
      is_exact,
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
    let ans = format!("Fx<{},{}>({})", self.m, self.b, self.val);
    write!(f, "{}", ans)
  }
}

#[allow(non_snake_case)]
pub fn to_Fx(x: f64, m: i32, b: i32, round: bool) -> Result<Fx, String> {
  let fx_q = crate::to_fixed(x, m, b - m - 1, round);
  match fx_q {
    Ok(fx) => Ok(Fx::new(fx.val, fx.m, fx.m + fx.n + 1, fx.is_exact)),
    Err(e) => Err(e),
  }
}

// /// Addition of Fx types belonging to different families
// /// e.g. Fx<5, 10> + Fx<2, 40> => Fx<5, 40> + Fx<5, 40> => Fx<6, 41>
// #[deprecated(since = "4.0.0")]
// pub fn add_incoherent(fx1: Fx, fx2: Fx) -> Fx {
//   // println!("{:?}", fx1);
//   // println!("{:?}", fx2);

//   let (m_new, b_new) = (std::cmp::max(fx1.m, fx2.m), std::cmp::max(fx1.b, fx2.b));
//   /*
//   let fx1_new = Fx::new(fx1.val << (b_new - fx1.b), m_new, b_new, false);
//   let fx2_new = Fx::new(fx2.val >> (m_new - fx2.m), m_new, b_new, false);
//   */
//   let fx1_new = to_Fx(fx1.eval(), m_new, b_new, true).unwrap();
//   let fx2_new = to_Fx(fx2.eval(), m_new, b_new, true).unwrap();

//   // println!("{:?}", fx1_new);
//   // println!("{:?}", fx2_new);

//   fx1_new + fx2_new
// }
