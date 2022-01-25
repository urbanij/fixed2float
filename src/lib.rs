mod fp;
pub use fp::FixedPoint;

const SIZE: u64 = 64;
const MANT_SIZE: u64 = 52;
const ES: u64 = 11;
const EXP_BIAS: u64 = (1 << (ES - 1)) - 1;

fn mask(size: u128) -> u128 {
    (1 << size) - 1
}

fn exp(bits: u64) -> u64 {
    ((bits & ((1 << (SIZE - 1)) - 1)) >> MANT_SIZE) & ((1 << MANT_SIZE) - 1)
}

fn mant(bits: u64) -> u64 {
    bits & ((1 << MANT_SIZE) - 1)
}

/// Convert `x` (f64) into fixed point format (Qm.n), if possible.
/// ```rust
/// use fixed2float::{to_fixed, FixedPoint};
/// assert_eq!(
///     to_fixed(1.5, 1, 3),
///     Ok(
///         FixedPoint {
///             val: 0b1100,
///             m: 1,
///             n: 3,
///             is_exact: true,
///         }
///     )
/// );
/// assert_eq!(to_fixed(1.5, 1, 3).unwrap().val, 0b1100);
/// assert_eq!(to_fixed(1.5, 1, 3).unwrap().is_exact, true);
/// ```
pub fn to_fixed(x: f64, m: i32, n: i32) -> Result<FixedPoint, String> {
    let f64_bits = x.to_bits();

    let exp = exp(f64_bits) as i32 - EXP_BIAS as i32;

    let mant_plus_one = (1 << MANT_SIZE) | mant(f64_bits); // Q1.MANT_SIZE

    let bits = mant_plus_one; // bits is mant_plus_one. the only thing that changes
                              // is where _you_ interpret the point to be, which depends on `exp` at this point.
                              // now all you have to do is slice out the fractional and non-fractional parts individually.

    let fractional_part = bits as u128 & mask((MANT_SIZE as i32 - exp as i32) as u128);
    let integer_part = bits >> ((MANT_SIZE as i32 - exp as i32) as u64);

    // now, depending on `m` and `n` you need to figure out whether rouding occurs.
    // if that's the case, that information is reported back to the user via the `is_exact` flag.
    // whereas if the integer part does not fit into `m` bits you return the Err variant instead.

    let integer_part_on_m_bits = integer_part as u128 & mask(m as u128);

    let mut fractional_part_on_n_bits = match (MANT_SIZE as i32 - exp as i32 - n as i32) >= 0 {
        true => (fractional_part >> (MANT_SIZE as i32 - exp as i32 - n as i32)) & (mask(n as u128)),
        _ => (fractional_part << (-(MANT_SIZE as i32 - exp as i32 - n as i32))) & mask(n as u128),
    };

    if integer_part_on_m_bits < integer_part as u128 {
        return Err(format!("Integer field does not fit into `m` {}.", m));
    }

    let round_bit = match ((MANT_SIZE as i32 - exp as i32) - (n as i32 + 1)) >= 0 {
        true => fractional_part >> ((MANT_SIZE as i32 - exp as i32) - (n as i32 + 1)) & 1 != 0,
        _ => fractional_part << (-((MANT_SIZE as i32 - exp as i32) - (n as i32 + 1))) != 0,
    };

    if round_bit {
        fractional_part_on_n_bits += 1;
    }

    let sticky_bit = match (MANT_SIZE as i32 - exp as i32 - n as i32) >= 0 {
        true => fractional_part & mask((MANT_SIZE as i32 - exp as i32 - n as i32) as u128) != 0,
        _ => false,
    };

    let is_exact = !sticky_bit && !round_bit;
    let ans = (integer_part_on_m_bits << n) + fractional_part_on_n_bits;
    Ok(FixedPoint {
        val: ans,
        m,
        n,
        is_exact,
    })
}

/// Compute the real value represented by `bits` (str) in the form Qm.n.
/// ```rust
/// use fixed2float::to_float_str;
/// assert_eq!(to_float_str("00010011000000100001", 12, 8), Ok(304.12890625));
/// ```
pub fn to_float_str(bits: &str, m: i32, n: i32) -> Result<f64, String> {
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

/// Compute the real value represented by `bits` (unsigned) in the form Qm.n.
/// ```rust
/// use fixed2float::to_float;
/// assert_eq!(to_float(0x13021, 20, 12, 8), Ok(304.12890625));
/// ```
pub fn to_float(mut bits: u128, size: i32, m: i32, n: i32) -> Result<f64, String> {
    if size != m + n {
        return Err(format!(
            "`bits` size  does not match the `m` + `n` size you specified. {} != {}",
            size,
            m + n
        ));
    }

    let mut ans = 0.0;

    for i in (1..=n).rev() {
        ans += (bits & 1) as f64 / 2_u64.pow(i as u32) as f64; //  (1 << i) as f64;
        bits >>= 1;
    }
    for i in 0..m {
        ans += (bits & 1) as f64 * 2_u64.pow(i as u32) as f64; // (1 << i) as f64;
        bits >>= 1;
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::{to_fixed, to_float, to_float_str};

    #[test]
    fn test_to_float() {
        assert_eq!(to_float(0b1010000010110000, 16, 1, 15), Ok(1.25537109375));
        assert_eq!(to_float(0b1010000010110000, 16, 1, 14).is_err(), true);
        assert_eq!(to_float(0b1010000010110000, 16, 1, 15).is_err(), false);
        assert_eq!(to_float(0b1010000010110000, 16, 1, 16).is_err(), true);
        assert_eq!(to_float_str("1010000010110000", 1, 15), Ok(1.25537109375));
        assert_eq!(to_float_str("1010000010110000", 1, 14).is_err(), true);
        assert_eq!(to_float_str("1010000010110000", 1, 15).is_err(), false);
        assert_eq!(to_float_str("1010000010110000", 1, 16).is_err(), true);
    }

    #[test]
    fn test_to_fixed() {
        use super::fp::FixedPoint;

        assert_eq!(to_fixed(10.25, 4, 3), Ok(FixedPoint::new(82, 4, 3, true)));
        assert_eq!(to_fixed(10.25, 3, 3).is_err(), true);
        assert_eq!(to_fixed(10.25, 8, 3), Ok(FixedPoint::new(82, 8, 3, true)));
        assert_eq!(to_fixed(10.25, 8, 2), Ok(FixedPoint::new(41, 8, 2, true)));
        assert_eq!(to_fixed(10.25, 8, 1), Ok(FixedPoint::new(21, 8, 1, false)));
        assert_eq!(to_fixed(10.25, 8, 0), Ok(FixedPoint::new(10, 8, 0, false)));
        assert_eq!(
            to_fixed(0.0078125, 1, 1),
            Ok(FixedPoint::new(0, 1, 1, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 2),
            Ok(FixedPoint::new(0, 1, 2, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 3),
            Ok(FixedPoint::new(0, 1, 3, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 4),
            Ok(FixedPoint::new(0, 1, 4, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 5),
            Ok(FixedPoint::new(0, 1, 5, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 6),
            Ok(FixedPoint::new(1, 1, 6, false))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 7),
            Ok(FixedPoint::new(1, 1, 7, true))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 8),
            Ok(FixedPoint::new(2, 1, 8, true))
        );
        assert_eq!(
            to_fixed(0.0078125, 1, 9),
            Ok(FixedPoint::new(4, 1, 9, true))
        );
        assert_eq!(to_fixed(1.387, 2, 15).unwrap().val, 45449);
        assert_eq!(to_fixed(4.3, 2, 15).is_err(), true);
    }

    #[test]
    fn back_and_forth() {
        let x = 10.25;
        let (m, n) = (21, 3);
        assert_eq!(
            to_float(to_fixed(x, m, n).unwrap().val as u128, 24, m, n).unwrap(),
            x
        );
    }
}
