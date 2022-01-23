use fixed2float::{to_fixed, to_float, to_float_str};

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
    println!("{:?}", to_fixed(x, m, n).unwrap().0);
}
