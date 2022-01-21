/// Convert `x` f64 into fixed point format `Qm.n`, if possible.
pub fn to_fixed(x: f64, m: u8, n: u8) -> Option<u64> {
    unimplemented!();
    let mut integer = 0_u64;
    let mut fractional = 0_u64;

    let mut x_int = x as i32;
    let mut x_frac = x - x_int as f64;

    for i in 0.. {
        break;
    }

    for i in 0.. {
        break;
    }

    Some((integer << n) + fractional)
}

/// Convert `bits` in the format `Qm.n` into a real number.
pub fn to_float(mut bits: u64, m: u8, n: u8) -> f64 {
    let mut ans = 0.0;

    for i in (0..n + 1).rev() {
        ans += (bits & 1) as f64 / (1 << i) as f64;
        bits >>= 1;
    }

    for i in 0..(m) {
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
        let bits = 0b1010000010110000;
        let (m, n) = (1, 15);
        assert_eq!(to_float(bits, m, n), 1.25537109375);
    }
}
