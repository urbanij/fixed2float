mod fp_fx;
mod fp_q;
pub use fp_fx::{to_Fx, Fx};
pub use fp_q::{to_Q, Q};

pub trait FixedPoint {
    fn eval(&self) -> f64;
}

#[cfg(test)]
mod test {

    use crate::fixed_point::Q;

    #[test]
    fn test_add() {
        let fp1 = Q::new(0b1111, 3, 1, true);
        let fp2 = Q::new(0b1110, 3, 1, true);
        let fp3 = Q::new(0b11101, 4, 1, true);
        assert_eq!(fp1 + fp2, fp3);
    }

    #[test]
    fn test_sub() {
        let fp1 = Q::new(0b1111, 3, 1, true);
        let fp2 = Q::new(0b1110, 3, 1, true);
        let fp3 = Q::new(0b0001, 3, 1, true);
        assert_eq!(fp1 - fp2, fp3);
    }

    #[test]
    fn test_shift() {
        let fp1 = Q::new(0b1111, 3, 1, true);
        assert_eq!(fp1 << 1, Q::new(0b1110, 3, 1, true));
        assert_eq!(fp1 << 2, Q::new(0b1100, 3, 1, true));
        assert_eq!(fp1 << 3, Q::new(0b1000, 3, 1, true));
        assert_eq!(fp1 >> 1, Q::new(0b0111, 3, 1, true));
    }

    // #[test]
    // #[ignore]
    // fn test_index_right_idx_0() {
    //     let fp1 = Q::new(0b10010011, 6, 2, true); // 36.75
    //     let size = (fp1.m + fp1.n) as usize;

    //     assert_eq!(fp1.index(size..0).is_err(), true);
    //     assert_eq!(fp1.index(size - 1..0), Ok(Q::new(0b10010011, 6, 2, true)));
    //     assert_eq!(fp1.index(size - 2..0), Ok(Q::new(0b0010011, 5, 2, true)));
    //     assert_eq!(fp1.index(size - 3..0), Ok(Q::new(0b010011, 4, 2, true)));
    //     assert_eq!(fp1.index(size - 4..0), Ok(Q::new(0b10011, 3, 2, true)));
    //     assert_eq!(fp1.index(size - 5..0), Ok(Q::new(0b0011, 2, 2, true)));
    //     assert_eq!(fp1.index(size - 6..0), Ok(Q::new(0b011, 1, 2, true)));
    //     assert_eq!(fp1.index(size - 7..0), Ok(Q::new(0b11, 0, 2, true)));

    //     // assert_eq!(
    //     //     fp1.index(size - 8..0),
    //     //     Ok(Q::new(0b1, 0, 1, true))
    //     // );

    //     // assert_eq!(fp1.index(size - 9..0).is_err(), true);
    // }

    // #[test]
    // #[ignore]
    // fn test_index_left_idx_max() {
    //     let fp1 = Q::new(0b10010011, 6, 2, true); // 36.75
    //     let size = (fp1.m + fp1.n) as usize;

    //     assert_eq!(fp1.index(size..0).is_err(), true);

    //     assert_eq!(fp1.index(size - 1..0), Ok(Q::new(0b10010011, 6, 2, true)));

    //     assert_eq!(fp1.index(size - 1..1), Ok(Q::new(0b1001001, 6, 1, true)));

    //     assert_eq!(fp1.index(size - 1..2), Ok(Q::new(0b100100, 6, 0, true)));

    //     assert_eq!(fp1.index(size - 1..3), Ok(Q::new(0b10010, 5, 0, true)));

    //     assert_eq!(fp1.index(size - 1..4), Ok(Q::new(0b1001, 4, 0, true)));

    //     assert_eq!(fp1.index(size - 1..5), Ok(Q::new(0b100, 3, 0, true)));

    //     assert_eq!(fp1.index(size - 1..6), Ok(Q::new(0b10, 2, 0, true)));

    //     // assert_eq!(
    //     //     fp1.index(size-1..7),
    //     //     Ok(Q::new(0b1, 1, 0, true))
    //     // );

    //     assert_eq!(fp1.index(size - 8..8).is_err(), true);
    // }
}
