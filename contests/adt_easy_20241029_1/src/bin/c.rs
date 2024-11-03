use proconio::input;
use regex::Regex;

fn main() {
  input! {
    s: String
  }

  let pattern = Regex::new(r"^[A-Z][1-9][0-9]{5}[A-Z]$").unwrap();
  let result = match pattern.is_match(&s) {
      true => "Yes",
      false => "No"
  };

  println!("{}", result)
}
