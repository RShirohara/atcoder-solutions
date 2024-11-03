use proconio::input;
use regex::Regex;

fn main() {
  input! {
    s: String
  }

  let pattern = Regex::new(r"^<={1,98}>$").unwrap();
  let result = match pattern.is_match(&s) {
      true => "Yes",
      false => "No"
  };

  println!("{}", result)
}
