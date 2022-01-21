use fixed2float::{to_fixed, to_float};

fn main() {
    let bits = 0b1010000010110000;

    let (m, n) = (1, 15);

    let x = to_float(bits, m, n);
    println!("{}", x);

    // println!("{:?}", to_fixed(10.25, 8, 1));

    println!("{:?}", to_fixed(0.65, 1, 15));
}
