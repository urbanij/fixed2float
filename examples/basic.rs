use fixed2float::{to_Fx, to_Q, to_float, to_float_str, FixedPoint, Fx, Q};

fn main() {
  let bits = "1010000010110000";
  let (m, n) = (1, 15);

  let b1 = to_Q(0.0, 1, 5, true).unwrap();
  println!("{b1:?}");

  let x = to_float_str(bits, m, n);
  println!("bits = {} -> {:?}", bits, x);

  let bits = 0b1010000010110000;
  let x = to_float(bits, 16, m, n);
  println!("{:?}", x);

  let x = 10.25;
  let (m, n) = (21, 3);

  let fx0 = to_Q(x, m, n, true).unwrap();
  println!("{:?}", fx0);

  let fx1 = Q::new(0b10010011, 6, 2, true); // 36.75
  let fx2 = Fx::new(0b10010011, 6, 8, true); // 36.75

  println!("{:?} {} {}", fx1, fx1, fx1.eval());
  println!("{:?} {} {}", fx2, fx2, fx2.eval());

  // let fx1_sliced = fx1.index(7..0);
  // println!("{:?}", fx1_sliced);

  let fx33 = to_Fx(0.000244140625, 0, 12, true);
  println!("{fx33:?}");
}
