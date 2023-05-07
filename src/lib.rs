//!
//! Fixed point to float an vice versa conversion utility.
//! Use the Q (Qm.n) and the VisSim (Fxm.b) [notations](https://en.wikipedia.org/wiki/Fixed-point_arithmetic#Notations).
//!
//!
mod fixed_point;
pub use fixed_point::FixedPoint;
pub use fixed_point::{to_Fx, to_Q};
pub use fixed_point::{Fx, Q};

pub type UInt = u128;

const SIZE: u64 = 64;
const MANT_SIZE: u64 = 52;
const ES: u64 = 11;
const EXP_BIAS: u64 = (1 << (ES - 1)) - 1;

fn mask(size: u32) -> u128 {
  match 1_u128.checked_shl(size) {
    Some(v) => v - 1,
    None => 0,
  }
}

fn sign(bits: u64) -> u64 {
  bits >> (SIZE - 1)
}

fn exp(bits: u64) -> u64 {
  ((bits & ((1 << (SIZE - 1)) - 1)) >> MANT_SIZE) & ((1 << MANT_SIZE) - 1)
}

fn mant(bits: u64) -> u64 {
  bits & ((1 << MANT_SIZE) - 1)
}

/// Convert `x` (f64) into fixed point format (Qm.n), if possible.
/// ```rust
/// use fixed2float::{to_Q, Q};
/// use fixed2float::to_fixed;
/// assert_eq!(
///     to_Q(1.5, 1, 3, true),
///     Ok(
///         Q {
///             val: 0b1100,
///             m: 1,
///             n: 3,
///             is_exact: true,
///         }
///     )
/// );
/// assert_eq!(
///     to_Q(-2.5, 3, 3, true),
///     Ok(
///         Q {
///             val: 0b1101100,
///             m: 3,
///             n: 3,
///             is_exact: true,
///         }
///     )
/// );
/// assert_eq!(to_Q(1.5, 1, 3, true).unwrap().val, 0b1100);
/// assert_eq!(to_Q(0.0, 1, 5, true).unwrap().val, 0);
/// assert_eq!(to_Q(1.5, 1, 3, true).unwrap().is_exact, true);
///
/// assert_eq!(to_fixed(-2.5, 3, 3, false).unwrap().val, 108);
/// ```
pub fn to_fixed(x: f64, m: i32, n: i32, round: bool) -> Result<Q, String> {
  let f64_bits = x.to_bits();

  let sign = sign(f64_bits);

  if x.abs() == 0.0 {
    return Ok(Q {
      val: (sign << (m + n)) as UInt,
      m,
      n,
      is_exact: true,
    });
  }

  let exp = exp(f64_bits) as i32 - EXP_BIAS as i32;

  let mant_plus_one = (1 << MANT_SIZE) | mant(f64_bits); // Q1.MANT_SIZE

  let bits = mant_plus_one; // bits is mant_plus_one. the only thing that changes
                            // is where _you_ interpret the point to be, which depends on `exp` at this point.
                            // now all you have to do is slice out the fractional and non-fractional parts individually.

  let fractional_part = bits as UInt & mask((MANT_SIZE as i32 - exp) as u32) as UInt;
  let integer_part = bits
    .checked_shr((MANT_SIZE as i32 - exp) as u32)
    .unwrap_or(0);

  // now, depending on `m` and `n` you need to figure out whether rouding occurs.
  // if that's the case, that information is reported back to the user via the `is_exact` flag.
  // whereas if the integer part does not fit into `m` bits you return the Err variant instead.

  let integer_part_on_m_bits = integer_part as UInt & mask(m as u32) as UInt;

  let mut fractional_part_on_n_bits = match (MANT_SIZE as i32 - exp - n) >= 0 {
    true => (fractional_part >> (MANT_SIZE as i32 - exp - n) as u32) & (mask(n as u32) as UInt),
    _ => (fractional_part << (-(MANT_SIZE as i32 - exp - n))) & (mask(n as u32) as UInt),
  };

  if integer_part_on_m_bits < integer_part as UInt {
    return Err(format!(
      "Error: Integer field does not fit into `m` = {} bits.",
      m
    ));
  }

  let _len = (MANT_SIZE as i32 - exp) - (n + 1);
  let round_bit = match _len >= 0 {
    true => fractional_part >> (_len) & 1 != 0,
    _ => fractional_part.checked_shl(-_len as u32).unwrap_or(0) != 0,
  };

  if round && round_bit {
    fractional_part_on_n_bits += 1;
  }

  let sticky_bit = match (MANT_SIZE as i32 - exp - n) >= 0 {
    true => fractional_part & mask((MANT_SIZE as i32 - exp - n) as u32) as UInt != 0,
    _ => false,
  };

  let is_exact = !sticky_bit && !round_bit;
  let ans_signless = ((integer_part_on_m_bits) << n) + fractional_part_on_n_bits;

  let ans = match sign == 0 {
    true => ans_signless,
    false => (!ans_signless + 1 as UInt) & mask((m + n + 1) as u32),
  };

  Ok(Q {
    val: ans,
    m,
    n,
    is_exact,
  })
}

#[deprecated(since="0.4.0")]
/// Compute the real value represented by `bits` (str).
/// ```rust
/// use fixed2float::to_float_str;
/// assert_eq!(to_float_str("00010011000000100001", 12, 20), Ok(304.12890625));
/// ```
pub fn to_float_str(bits: &str, m: i32, b: i32) -> Result<f64, String> {
  let n = b - m;
  let bits_size = bits.len() as i32;
  if bits_size != m + n {
    return Err(format!(
      "`bits` size  does not match the `m` + `n` size you specified. {} != {}",
      bits_size,
      m + n
    ));
  }

  let mut ans = 0.0;

  for i in (1..=n).rev() {
    let bit = bits
      .chars()
      .nth(((m - 1 + i) as u16).into())
      .unwrap()
      .to_digit(2)
      .unwrap(); //. parse::<i32>().unwrap();
    ans += bit as f64 / (1 << i) as f64;
  }

  for i in 0..m {
    let bit = bits
      .chars()
      .nth(((m - 1 - i) as u16).into())
      .unwrap()
      .to_digit(2)
      .unwrap();
    ans += bit as f64 * (1 << i) as f64;
  }

  Ok(ans)
}

/// Compute the real value represented by `bits`.
/// ```rust
/// use fixed2float::to_float;
/// assert_eq!(to_float(0b0_000100110000_00100001, 21, 12, 8), Ok(304.12890625));
/// ```
pub fn to_float(bits: UInt, size: i32, m: i32, n: i32) -> Result<f64, String> {
  if size != m + n + 1 {
    return Err(format!(
      "`bits` size  does not match the (`m` + `n` + 1) size you specified. {} != {}",
      size,
      m + n + 1
    ));
  }

  let sign = (bits >> (m + n)) as u32;

  let mut bits = match sign == 0 {
    true => bits,
    false => (!bits + 1) & mask(size as u32),
  };

  let mut ans = 0.0;

  for i in (1..=n).rev() {
    ans += match 2_i128.checked_pow(i as u32) {
      None => 0.0,
      Some(v) => (bits & 1) as f64 / v as f64,
    };

    //2_i128.pow (i as u32) as f64; //  (1 << i) as f64;
    bits >>= 1;
  }
  for i in 0..m {
    ans += (bits & 1) as f64 * 2_i128.pow(i as u32) as f64; // (1 << i) as f64;
    bits >>= 1;
  }

  let ans = match sign == 0 {
    true => ans,
    false => -ans,
  };

  Ok(ans)
}

#[cfg(test)]
mod tests {
  use super::{to_fixed, to_float, to_float_str};

  #[test]
  fn test_to_float() {
    assert_eq!(to_float(0b01010000010110000, 17, 1, 15), Ok(1.25537109375));
    assert_eq!(to_float(0b01010000010110000, 17, 1, 14).is_err(), true);
    assert_eq!(to_float(0b01010000010110000, 17, 1, 15).is_err(), false);
    assert_eq!(to_float(0b01010000010110000, 17, 1, 16).is_err(), true);
    // assert_eq!(to_float_str("1010000010110000", 1, 16), Ok(1.25537109375));
    // assert_eq!(to_float_str("1010000010110000", 1, 15).is_err(), true);
    // assert_eq!(to_float_str("1010000010110000", 1, 16).is_err(), false);
    // assert_eq!(to_float_str("1010000010110000", 1, 17).is_err(), true);
  }

  #[test]
  fn test_to_fixed() {
    use super::fixed_point::Q;

    // assert_eq!(to_fixed(10.25, 4, 3, true), Ok(Q::new(82, 4, 3, true)));
    // assert_eq!(to_fixed(10.25, 3, 3, true).is_err(), true);
    // assert_eq!(to_fixed(10.25, 8, 3, true), Ok(Q::new(82, 8, 3, true)));
    // assert_eq!(to_fixed(10.25, 8, 2, true), Ok(Q::new(41, 8, 2, true)));
    // assert_eq!(to_fixed(10.25, 8, 1, true), Ok(Q::new(21, 8, 1, false)));
    // assert_eq!(to_fixed(10.25, 8, 0, true), Ok(Q::new(10, 8, 0, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 1, true), Ok(Q::new(0, 1, 1, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 2, true), Ok(Q::new(0, 1, 2, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 3, true), Ok(Q::new(0, 1, 3, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 4, true), Ok(Q::new(0, 1, 4, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 5, true), Ok(Q::new(0, 1, 5, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 6, true), Ok(Q::new(1, 1, 6, false)));
    // assert_eq!(to_fixed(0.0078125, 1, 7, true), Ok(Q::new(1, 1, 7, true)));
    // assert_eq!(to_fixed(0.0078125, 1, 8, true), Ok(Q::new(2, 1, 8, true)));
    // assert_eq!(to_fixed(0.0078125, 1, 9, true), Ok(Q::new(4, 1, 9, true)));
    // assert_eq!(to_fixed(1.387, 2, 15, true).unwrap().val, 45449);
    // assert_eq!(to_fixed(4.3, 2, 15, true).is_err(), true);

    assert_eq!(to_fixed(4.0, 4, 2, false).unwrap().val, 0b0_0100_00);
    assert_eq!(to_fixed(-4.0, 4, 2, false).unwrap().val, 0b1_1100_00);
    assert_eq!(to_fixed(8.75, 4, 2, false).unwrap().val, 0b0_1000_11);
    assert_eq!(to_fixed(9.5, 4, 2, false).unwrap().val, 0b0_1001_10);
    assert_eq!(to_fixed(15.75, 4, 2, false).unwrap().val, 0b0_1111_11);
    assert_eq!(to_fixed(15.8, 4, 2, false).unwrap().val, 0b0_1111_11);
    assert_eq!(to_fixed(16.0, 4, 2, false).is_err(), true);
    assert_eq!(to_fixed(0.0, 2, 1, false).unwrap().val, 0b0_00_0);
    assert_eq!(to_fixed(-0.0, 2, 1, false).unwrap().val, 0b1_00_0);
  }

  #[test]
  fn back_and_forth() {
    // 0_1_010
    let x = 1.25;
    let (m, n) = (1, 3);
    assert_eq!(
      to_float(to_fixed(x, m, n, false).unwrap().val, 5, m, n).unwrap(),
      x
    );
  }
}
