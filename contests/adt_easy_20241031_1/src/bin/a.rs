use proconio::input;
use regex::Regex;

fn main() {
    input! {
      s: String
    }

    let pattern = Regex::new(r"^[A-Z][a-z]*$").unwrap();
    let result = match pattern.is_match(&s) {
        true => "Yes",
        false => "No",
    };

    println!("{}", result);
}
