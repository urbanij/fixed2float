use fixed2float::{to_fixed, to_float, to_float_str, FixedPoint};

fn main() {
    let bits = "1010000010110000";
    let (m, n) = (1, 15);

    let x = to_float_str(bits, m, n);
    println!("bits = {} -> {:?}", bits, x);

    let bits = 0b1010000010110000;
    let x = to_float(bits, 16, m, n);
    println!("{:?}", x);

    let x = 10.25;
    let (m, n) = (21, 3);
    println!("{:?}", to_fixed(x, m, n).unwrap().val);

    let fp1 = FixedPoint::new(0b10010011, 6, 2, true); // 36.75

    println!("{:?} {} {}", fp1, fp1, fp1.eval());

    // let fp1_sliced = fp1.index(7..0);
    // println!("{:?}", fp1_sliced);
}
