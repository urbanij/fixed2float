const SIZE: u64 = 64;
const MANT_SIZE: u64 = 52;
const ES: u64 = 11;
const EXP_BIAS: u64 = (1 << (ES - 1)) - 1;

fn mask(size: u64) -> u64 {
    (1 << size) - 1
}

fn exp(bits: u64) -> u64 {
    ((bits & ((1 << (SIZE - 1)) - 1)) >> MANT_SIZE) & ((1 << MANT_SIZE) - 1)
}

fn mant(bits: u64) -> u64 {
    bits & ((1 << MANT_SIZE) - 1)
}

/// Convert `x` f64 into fixed point format `Qm.n`, if possible.
/// ```rust
/// use fixed2float as f2f;
/// assert_eq!(f2f::to_fixed(1.5, 1, 3).unwrap(), (0b1100, true));
/// ```
pub fn to_fixed(x: f64, m: u8, n: u8) -> Result<(u64, bool), String> {
    let f64_bits = x.to_bits();

    let exp = exp(f64_bits) as i32 - EXP_BIAS as i32;

    let mant_plus_one = (1 << MANT_SIZE) | mant(f64_bits); // Q1.MANT_SIZE

    let bits = mant_plus_one; // bits is mant_plus_one. the only thing that changes
                              // is where _you_ interpret the point to be, which depends on `exp` at this point.
                              // now all you have to do is slice out the fractional and non-fractional parts individually.

    let fractional_part = bits & mask((MANT_SIZE as i32 - exp as i32) as u64);
    let integer_part = bits >> ((MANT_SIZE as i32 - exp as i32) as u64);

    // now, depending on `m` and `n` you need to figure out whether rouding occurs.
    // if that's the case, that information is reported back to the user via the `is_exact` flag.
    // whereas if the integer part does not fit into `m` bits you return the Err variant instead.

    let integer_part_on_m_bits = integer_part & mask(m as u64);

    let mut fractional_part_on_n_bits = (fractional_part
        & (mask(n as u64) << ((MANT_SIZE as i32 - exp as i32) - n as i32)))
        >> ((MANT_SIZE as i32 - exp as i32) - n as i32);

    if integer_part_on_m_bits < integer_part {
        return Err("Integer field does not fit into `m`.".to_string());
    }

    let round_bit = fractional_part >> ((MANT_SIZE as i32 - exp as i32) - (n as i32 + 1)) & 1 != 0;

    if round_bit {
        fractional_part_on_n_bits += 1;
    }

    let sticky_bit =
        fractional_part & mask(((MANT_SIZE as i32 - exp as i32) - (n as i32 + 1)) as u64) != 0;

    let is_exact = !sticky_bit && !round_bit;
    let ans = (integer_part_on_m_bits << n) + fractional_part_on_n_bits;
    Ok((ans, is_exact))
}

/// Convert `bits` in the format `Qm.n` into a real number.
/// ```rust
/// use fixed2float as f2f;
/// assert_eq!(f2f::to_float(0x13021, 12, 8), 304.12890625);
/// ```
pub fn to_float(mut bits: u64, m: u8, n: u8) -> f64 {
    let mut ans = 0.0;

    for i in (1..n + 1).rev() {
        ans += (bits & 1) as f64 / (1 << i) as f64;
        bits >>= 1;
    }

    for i in 0..m {
        ans += (bits & 1) as f64 * (1 << i) as f64;
        bits >>= 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::{to_fixed, to_float};

    #[test]
    fn test_to_float() {
        assert_eq!(to_float(0b1010000010110000, 1, 15), 1.25537109375);
    }

    #[test]
    fn test_to_fixed() {
        assert_eq!(to_fixed(10.25, 4, 3), Ok((82, true)));
        assert_eq!(to_fixed(10.25, 3, 3).is_err(), true);
        assert_eq!(to_fixed(10.25, 8, 2), Ok((41, true)));
        assert_eq!(to_fixed(10.25, 8, 1), Ok((21, false)));
        assert_eq!(to_fixed(1.387, 2, 15).unwrap().0, 45449);
        assert_eq!(to_fixed(4.3, 2, 15).is_err(), true);
    }

    #[test]
    fn back_and_forth() {
        let x = 10.25;
        let (m, n) = (21, 3);
        assert_eq!(to_float(to_fixed(x, m, n).unwrap().0, m, n), x);
    }
}
