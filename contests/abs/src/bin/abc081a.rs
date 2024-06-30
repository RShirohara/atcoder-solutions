use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  }
  let result = s.iter().filter(|c| c.to_string() == "1").count();
  println!("{}", result);
}
