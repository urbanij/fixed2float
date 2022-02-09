use super::FixedPoint;
use crate::{mask, to_float};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Fx {
    pub val: u128,
    pub m: i32,
    pub b: i32,
    pub is_exact: bool,
}

impl Fx {
    pub fn new(val: u128, m: i32, b: i32, is_exact: bool) -> Self {
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
        to_float(self.val, self.b, self.m, self.b - self.m).unwrap()
    }
}

impl std::ops::Shl<u32> for Fx {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            val: (self.val << rhs) & mask((self.b) as u128),
            m: self.m,
            b: self.b,
            is_exact: self.is_exact,
        }
    }
}

impl std::ops::Shr<u32> for Fx {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            val: (self.val >> rhs) & mask((self.b) as u128),
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
            panic!("`m` and `n` field of each FP obj has to match.")
        }
        Self {
            val: self.val + rhs.val,
            m: self.m + 1,
            b: self.b + 1,
            is_exact: true,
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
        const ANSI_RESET_COLOR: &str = "\x1b[0m";
        const ANSI_BLACK: &str = "\x1b[37;40m"; // bold, black background, white foreground
        const ANSI_MAGENTA: &str = "\x1b[45m"; // non bold, magenta background, black foreground

        let bits = format!("{:0width$b}", self.val, width = (self.b) as usize);

        let dots = if self.is_exact { "" } else { "..." };

        let ans = format!(
            "{ANSI_MAGENTA}{int}{ANSI_BLACK}{frac}{dots}{ANSI_RESET_COLOR}",
            int = &bits[..self.m as usize],
            frac = &bits[self.m as usize..],
        );

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
pub fn to_Fx(x: f64, m: i32, b: i32) -> Result<Fx, String> {
    let fp_q = crate::to_fixed(x, m, b - m);
    match fp_q {
        Ok(fp) => Ok(Fx::new(fp.val, fp.m, fp.m + fp.n, fp.is_exact)),
        Err(e) => Err(format!("{}", e)),
    }
}
