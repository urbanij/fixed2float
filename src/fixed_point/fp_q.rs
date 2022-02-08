use super::FixedPoint;
use crate::{mask, to_float};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Q {
    pub val: u128,
    pub m: i32,
    pub n: i32,
    pub is_exact: bool,
}

impl Q {
    pub fn new(val: u128, m: i32, n: i32, is_exact: bool) -> Self {
        Self {
            val,
            m,
            n,
            is_exact,
        }
    }
}

impl FixedPoint for Q {
    fn eval(&self) -> f64 {
        to_float(self.val, self.m + self.n, self.m, self.n).unwrap()
    }

    // fn index(&self, _range: std::ops::Range<usize>) -> Result<Self, String> {
    //         use super::mask;

    //         let (left_idx, right_idx) = (range.start + 1, range.end);
    //         if left_idx < right_idx {
    //             return Err("Left index can't be larger than right index".to_string());
    //         }
    //         let range_size = (left_idx - right_idx) as i32;

    //         if left_idx as i32 > self.m + self.n {
    //             return Err("Left index can't be larger than fixed point number itself".to_string());
    //         }
    //         if range_size == 0 {
    //             return Err("Range can't be null".to_string());
    //         }

    //         let new_val = (self.val >> right_idx) & mask(left_idx as u128);
    //         let new_n = std::cmp::max(0, self.n - right_idx as i32 );
    //         let new_m = range_size - new_n;
    //         Ok(Self {
    //             val: new_val,
    //             m: new_m,
    //             n: new_n,
    //             is_exact: true,
    //         })
    //     unimplemented!()
    // }
}

impl std::ops::Shl<u32> for Q {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            val: (self.val << rhs) & mask((self.m + self.n) as u128),
            m: self.m,
            n: self.n,
            is_exact: self.is_exact,
        }
    }
}

impl std::ops::Shr<u32> for Q {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            val: (self.val >> rhs) & mask((self.m + self.n) as u128),
            m: self.m,
            n: self.n,
            is_exact: self.is_exact,
        }
    }
}

impl std::ops::Add for Q {
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

impl std::ops::Sub for Q {
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

impl std::ops::Mul for Q {
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

impl std::fmt::Debug for Q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ANSI_RESET_COLOR: &str = "\x1b[0m";
        const ANSI_BLACK: &str = "\x1b[37;40m"; // bold, black background, white foreground
        const ANSI_MAGENTA: &str = "\x1b[45m"; // non bold, magenta background, black foreground

        let bits = format!("{:0width$b}", self.val, width = (self.m + self.n) as usize);

        let dots = if self.is_exact { "" } else { "..." };

        let ans = format!(
            "{ANSI_MAGENTA}{int}{ANSI_BLACK}{frac}{dots}{ANSI_RESET_COLOR}",
            int = &bits[..self.m as usize],
            frac = &bits[self.m as usize..],
        );

        write!(f, "{}", ans)
    }
}

impl std::fmt::Display for Q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ans;

        ans = format!("Q<{},{}>({})", self.m, self.n, self.val);
        // ans = self.q_fmt();

        write!(f, "{}", ans)
    }
}

#[allow(non_snake_case)]
pub fn to_Q(x: f64, m: i32, n: i32) -> Result<Q, String> {
    crate::to_fixed(x, m, n)
}
