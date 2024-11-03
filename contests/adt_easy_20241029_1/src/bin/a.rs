use proconio::input;

fn main() {
  input! {
    a: [usize; 10]
  }

  let mut digit = 0;
  for _ in 1..=3 {
    digit = a[digit];
  }

  println!("{}", digit);
}
