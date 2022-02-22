use fixed2float::{to_Fx, to_Q, to_float, to_float_str, FixedPoint, Fx, Q};

fn main() {
    let bits = "1010000010110000";
    let (m, n) = (1, 15);

    let b1 = to_Q(0.0, 1, 5, true).unwrap();

    let x = to_float_str(bits, m, n);
    println!("bits = {} -> {:?}", bits, x);

    let bits = 0b1010000010110000;
    let x = to_float(bits, 16, m, n);
    println!("{:?}", x);

    let x = 10.25;
    let (m, n) = (21, 3);

    let fp0 = to_Q(x, m, n, true).unwrap();
    println!("{:?}", fp0);

    let fp1 = Q::new(0b10010011, 6, 2, true); // 36.75
    let fp2 = Fx::new(0b10010011, 6, 8, true); // 36.75

    println!("{:?} {} {}", fp1, fp1, fp1.eval());
    println!("{:?} {} {}", fp2, fp2, fp2.eval());

    // let fp1_sliced = fp1.index(7..0);
    // println!("{:?}", fp1_sliced);

    let fp33 = to_Fx(0.000244140625, 0, 12, true);
    println!("{fp33:?}");
}
