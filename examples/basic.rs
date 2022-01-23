use fixed2float::{to_fixed, to_float, to_float_str};

fn main() {
    let bits = "1010000010110000";
    let (m, n) = (1, 15);

    let x = to_float_str(bits, m, n);
    println!("bits = {} -> {:?}", bits, x);

    let bits = 0b1010000010110000;
    let x = to_float(bits, 16, m, n);

    println!("{:?}", to_fixed(10.25, 8, 1));

    println!("{:?}", to_fixed(0.65, 1, 15));

    let x = 10.25;
    let (m, n) = (21, 3);
    println!("{:?}", to_fixed(x, m, n).unwrap().0);

    println!("{:?}", to_float_str("00010011000000100001", 12, 8));

    println!("{:?}", to_fixed(10.25, 8, 2));
}
