mod fx_fx;
mod fx_q;
use crate::UInt;
pub use fx_fx::{to_Fx, Fx};
pub use fx_q::{to_Q, Q};

pub trait FixedPoint {
  fn eval(&self) -> f64;
}

fn debug_print(val: UInt, m: i32, b: i32, is_exact: bool) -> String {
  const ANSI_RESET_COLOR: &str = "\x1b[0m";
  const ANSI_RED: &str = "\x1b[37;41m";
  const ANSI_BLACK: &str = "\x1b[37;40m"; // bold, black background, white foreground
  const ANSI_MAGENTA: &str = "\x1b[45m"; // non bold, magenta background, black foreground

  let bits = format!("{:0width$b}", val, width = (b) as usize);

  let dots = if is_exact { "" } else { "..." };

  let ans = format!(
    "{ANSI_RED}{sign}{ANSI_MAGENTA}{int}{ANSI_BLACK}{frac}{dots}{ANSI_RESET_COLOR}",
    sign = &bits[..1],
    int = &bits[1..(m + 1) as usize],
    frac = &bits[(m + 1) as usize..],
  );

  ans
}

#[cfg(test)]
mod test {

  use crate::fixed_point::{Fx, Q};

  #[test]
  fn test_add() {
    let fx1 = Q::new(0b1111, 3, 1, true);
    let fx2 = Q::new(0b1110, 3, 1, true);
    let fx3 = Q::new(0b11101, 4, 1, true);
    assert_eq!(fx1 + fx2, fx3);
  }

  #[test]
  fn test_sub() {
    let fx1 = Fx::new(0b0_111_1, 3, 5, true); // 7.5
    let fx2 = Fx::new(0b0_111_0, 3, 5, true); // 7.0
    let fx3 = Fx::new(0b0_000_1, 3, 5, true); // 0.5
    assert_eq!(fx1 - fx2, fx3);
  }

  #[test]
  fn test_shift() {
    let fx1 = Q::new(0b1111, 3, 1, true);
    assert_eq!(fx1 << 1, Q::new(0b1110, 3, 1, true));
    assert_eq!(fx1 << 2, Q::new(0b1100, 3, 1, true));
    assert_eq!(fx1 << 3, Q::new(0b1000, 3, 1, true));
    assert_eq!(fx1 >> 1, Q::new(0b0111, 3, 1, true));
  }

  // #[test]
  // #[ignore]
  // fn test_index_right_idx_0() {
  //     let fx1 = Q::new(0b10010011, 6, 2, true); // 36.75
  //     let size = (fx1.m + fx1.n) as usize;

  //     assert_eq!(fx1.index(size..0).is_err(), true);
  //     assert_eq!(fx1.index(size - 1..0), Ok(Q::new(0b10010011, 6, 2, true)));
  //     assert_eq!(fx1.index(size - 2..0), Ok(Q::new(0b0010011, 5, 2, true)));
  //     assert_eq!(fx1.index(size - 3..0), Ok(Q::new(0b010011, 4, 2, true)));
  //     assert_eq!(fx1.index(size - 4..0), Ok(Q::new(0b10011, 3, 2, true)));
  //     assert_eq!(fx1.index(size - 5..0), Ok(Q::new(0b0011, 2, 2, true)));
  //     assert_eq!(fx1.index(size - 6..0), Ok(Q::new(0b011, 1, 2, true)));
  //     assert_eq!(fx1.index(size - 7..0), Ok(Q::new(0b11, 0, 2, true)));

  //     // assert_eq!(
  //     //     fx1.index(size - 8..0),
  //     //     Ok(Q::new(0b1, 0, 1, true))
  //     // );

  //     // assert_eq!(fx1.index(size - 9..0).is_err(), true);
  // }

  // #[test]
  // #[ignore]
  // fn test_index_left_idx_max() {
  //     let fx1 = Q::new(0b10010011, 6, 2, true); // 36.75
  //     let size = (fx1.m + fx1.n) as usize;

  //     assert_eq!(fx1.index(size..0).is_err(), true);

  //     assert_eq!(fx1.index(size - 1..0), Ok(Q::new(0b10010011, 6, 2, true)));

  //     assert_eq!(fx1.index(size - 1..1), Ok(Q::new(0b1001001, 6, 1, true)));

  //     assert_eq!(fx1.index(size - 1..2), Ok(Q::new(0b100100, 6, 0, true)));

  //     assert_eq!(fx1.index(size - 1..3), Ok(Q::new(0b10010, 5, 0, true)));

  //     assert_eq!(fx1.index(size - 1..4), Ok(Q::new(0b1001, 4, 0, true)));

  //     assert_eq!(fx1.index(size - 1..5), Ok(Q::new(0b100, 3, 0, true)));

  //     assert_eq!(fx1.index(size - 1..6), Ok(Q::new(0b10, 2, 0, true)));

  //     // assert_eq!(
  //     //     fx1.index(size-1..7),
  //     //     Ok(Q::new(0b1, 1, 0, true))
  //     // );

  //     assert_eq!(fx1.index(size - 8..8).is_err(), true);
  // }
}
