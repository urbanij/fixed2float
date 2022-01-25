#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl std::ops::Index<std::ops::Range<usize>> for FixedPoint {
    type Output = Self;
    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        let left_idx = range.start;
        let right_idx = range.end;
        if left_idx < right_idx {
            panic!("Left index can't be larger than right index");
        }
        if range.len() as i32 > self.m + self.n {
            panic!("Range size can't be larger than fixed point number itself")
        }

        println!("{:?} {} {}", left_idx, right_idx, range.len());
        &self
    }
}

// impl std::ops::Index<Range<usize>> for FixedPoint {
//     type Output = Self;
//     fn index(&self, range: Range<usize>) -> &Self::Output {
//         &Self {
//             val: 3,
//         }
//     }
// }

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
}
