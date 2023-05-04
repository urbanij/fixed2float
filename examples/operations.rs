use fixed2float::{to_Fx, FixedPoint};

fn main() {
  let fx1 = to_Fx(10.2, 5, 10, true).unwrap();
  let fx2 = to_Fx(2.2, 2, 40, true).unwrap();

  // println!("{:?}", fx1.unwrap() + fx2.unwrap());

  let sum = fx1 + fx2;

  println!("{:?} {}", sum, sum.eval());
  println!("{:?} {}", sum.m, sum.b);
}
