#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FixedPoint {
    pub val: u128,
    pub m: i32,
    pub n: i32,
    pub is_exact: bool,
}

impl FixedPoint {
    pub fn new(val: u128, m: i32, n: i32, is_exact: bool) -> Self {
        Self {
            val,
            m,
            n,
            is_exact,
        }
    }

    pub fn eval(&self) -> f64 {
        use super::to_float;
        to_float(self.val, self.m + self.n, self.m, self.n).unwrap()
    }

    pub fn q_fmt(&self) -> String {
        let (m, n) = (self.m, self.n);
        format!("Q{m}.{n}")
    }
    pub fn index(&self, _range: std::ops::Range<usize>) -> Result<Self, String> {
        //     use super::mask;

        //     let (left_idx, right_idx) = (range.start + 1, range.end);
        //     if left_idx < right_idx {
        //         return Err("Left index can't be larger than right index".to_string());
        //     }
        //     let range_size = (left_idx - right_idx) as i32;

        //     if left_idx as i32 > self.m + self.n {
        //         return Err("Left index can't be larger than fixed point number itself".to_string());
        //     }
        //     if range_size == 0 {
        //         return Err("Range can't be null".to_string());
        //     }

        //     let new_val = (self.val >> right_idx) & mask(left_idx as u128);
        //     let new_n = std::cmp::max(0, self.n - right_idx as i32 );
        //     let new_m = range_size - new_n;
        //     Ok(Self {
        //         val: new_val,
        //         m: new_m,
        //         n: new_n,
        //         is_exact: true,
        //     })
        unimplemented!()
    }
}

impl std::ops::Shl<u32> for FixedPoint {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        use super::mask;
        Self {
            val: (self.val << rhs) & mask((self.m + self.n) as u128),
            m: self.m,
            n: self.n,
            is_exact: self.is_exact,
        }
    }
}

impl std::ops::Shr<u32> for FixedPoint {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        use super::mask;
        Self {
            val: (self.val >> rhs) & mask((self.m + self.n) as u128),
            m: self.m,
            n: self.n,
            is_exact: self.is_exact,
        }
    }
}

impl std::ops::Add for FixedPoint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.m != rhs.m || self.n != rhs.n {
            panic!("`m` and `n` field of each FP obj has to match.")
        }
        Self {
            val: self.val + rhs.val,
            m: self.m + 1,
            n: self.n,
            is_exact: true,
        }
    }
}

impl std::ops::Sub for FixedPoint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.eval() > self.eval() {
            unimplemented!()
        }
        Self {
            val: self.val - rhs.val,
            m: self.m,
            n: self.n,
            is_exact: true,
        }
    }
}

impl std::ops::Mul for FixedPoint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val * rhs.val,
            m: self.m + rhs.m,
            n: self.n + rhs.n,
            is_exact: true,
        }
    }
}

impl std::fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ANSI_RESET_COLOR: &str = "\x1b[0m";
        const ANSI_BLACK: &str = "\x1b[37;40m"; // bold, black background, white foreground
        const ANSI_MAGENTA: &str = "\x1b[45m"; // non bold, magenta background, black foreground

        let bits = format!("{:0width$b}", self.val, width = (self.m + self.n) as usize);

        let dots = if self.is_exact { "" } else { "..." };

        let ans = format!(
            "{ANSI_MAGENTA}{int}{ANSI_BLACK}{frac}{ANSI_RESET_COLOR}{dots}",
            int = &bits[..self.m as usize],
            frac = &bits[self.m as usize..],
        );

        write!(f, "{}", ans)
    }
}

impl std::fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ans;

        ans = format!("Q<{},{}>({})", self.m, self.n, self.val);
        // ans = self.q_fmt();

        write!(f, "{}", ans)
    }
}

#[cfg(test)]
mod test {
    use super::FixedPoint;

    #[test]
    fn test_add() {
        let fp1 = FixedPoint::new(0b1111, 3, 1, true);
        let fp2 = FixedPoint::new(0b1110, 3, 1, true);
        let fp3 = FixedPoint::new(0b11101, 4, 1, true);
        assert_eq!(fp1 + fp2, fp3);
    }

    #[test]
    fn test_sub() {
        let fp1 = FixedPoint::new(0b1111, 3, 1, true);
        let fp2 = FixedPoint::new(0b1110, 3, 1, true);
        let fp3 = FixedPoint::new(0b0001, 3, 1, true);
        assert_eq!(fp1 - fp2, fp3);
    }

    #[test]
    fn test_shift() {
        let fp1 = FixedPoint::new(0b1111, 3, 1, true);
        assert_eq!(fp1 << 1, FixedPoint::new(0b1110, 3, 1, true));
        assert_eq!(fp1 << 2, FixedPoint::new(0b1100, 3, 1, true));
        assert_eq!(fp1 << 3, FixedPoint::new(0b1000, 3, 1, true));
        assert_eq!(fp1 >> 1, FixedPoint::new(0b0111, 3, 1, true));
    }

    #[test]
    #[ignore]
    fn test_index_right_idx_0() {
        let fp1 = FixedPoint::new(0b10010011, 6, 2, true); // 36.75
        let size = (fp1.m + fp1.n) as usize;

        assert_eq!(fp1.index(size..0).is_err(), true);
        assert_eq!(
            fp1.index(size - 1..0),
            Ok(FixedPoint::new(0b10010011, 6, 2, true))
        );
        assert_eq!(
            fp1.index(size - 2..0),
            Ok(FixedPoint::new(0b0010011, 5, 2, true))
        );
        assert_eq!(
            fp1.index(size - 3..0),
            Ok(FixedPoint::new(0b010011, 4, 2, true))
        );
        assert_eq!(
            fp1.index(size - 4..0),
            Ok(FixedPoint::new(0b10011, 3, 2, true))
        );
        assert_eq!(
            fp1.index(size - 5..0),
            Ok(FixedPoint::new(0b0011, 2, 2, true))
        );
        assert_eq!(
            fp1.index(size - 6..0),
            Ok(FixedPoint::new(0b011, 1, 2, true))
        );
        assert_eq!(
            fp1.index(size - 7..0),
            Ok(FixedPoint::new(0b11, 0, 2, true))
        );

        // assert_eq!(
        //     fp1.index(size - 8..0),
        //     Ok(FixedPoint::new(0b1, 0, 1, true))
        // );

        // assert_eq!(fp1.index(size - 9..0).is_err(), true);
    }

    #[test]
    #[ignore]
    fn test_index_left_idx_max() {
        let fp1 = FixedPoint::new(0b10010011, 6, 2, true); // 36.75
        let size = (fp1.m + fp1.n) as usize;

        assert_eq!(fp1.index(size..0).is_err(), true);

        assert_eq!(
            fp1.index(size - 1..0),
            Ok(FixedPoint::new(0b10010011, 6, 2, true))
        );

        assert_eq!(
            fp1.index(size - 1..1),
            Ok(FixedPoint::new(0b1001001, 6, 1, true))
        );

        assert_eq!(
            fp1.index(size - 1..2),
            Ok(FixedPoint::new(0b100100, 6, 0, true))
        );

        assert_eq!(
            fp1.index(size - 1..3),
            Ok(FixedPoint::new(0b10010, 5, 0, true))
        );

        assert_eq!(
            fp1.index(size - 1..4),
            Ok(FixedPoint::new(0b1001, 4, 0, true))
        );

        assert_eq!(
            fp1.index(size - 1..5),
            Ok(FixedPoint::new(0b100, 3, 0, true))
        );

        assert_eq!(
            fp1.index(size - 1..6),
            Ok(FixedPoint::new(0b10, 2, 0, true))
        );

        // assert_eq!(
        //     fp1.index(size-1..7),
        //     Ok(FixedPoint::new(0b1, 1, 0, true))
        // );

        assert_eq!(fp1.index(size - 8..8).is_err(), true);
    }
}
