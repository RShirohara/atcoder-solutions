use proconio::input;

fn main() {
  input! {
    a: u64,
    b: u64
  }
  let result = if (a * b) % 2 == 0 {"Even"} else {"Odd"};
  println!("{}", result);
}
