use proconio::input;

fn main() {
  input! {
    n: usize,
    s: [String; n]
  }

  let mut sum = 0;
  for text in s.iter() {
    if text == "Takahashi" {
      sum += 1;
    }
  }
  println!("{}", sum);
}
