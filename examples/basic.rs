use fixed2float::{to_Q, to_float, to_float_str, FixedPoint, Fx, Q};

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

    let fp0 = to_Q(x, m, n).unwrap();
    println!("{:?}", fp0);

    let fp1 = Q::new(0b10010011, 6, 2, true); // 36.75
    let fp2 = Fx::new(0b10010011, 6, 8, true); // 36.75

    println!("{:?} {} {}", fp1, fp1, fp1.eval());
    println!("{:?} {} {}", fp2, fp2, fp2.eval());

    // let fp1_sliced = fp1.index(7..0);
    // println!("{:?}", fp1_sliced);
}
