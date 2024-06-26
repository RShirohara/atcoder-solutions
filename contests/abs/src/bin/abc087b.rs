use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    c: usize,
    x: usize
  }
  let mut count = 0;
  for a in 0..a + 1 {
    for b in 0..b + 1 {
      for c in 0..c + 1 {
        if a * 500 + b * 100 + c * 50 == x {
          count += 1;
        }
      }
    }
  }
  println!("{}", count);
}
