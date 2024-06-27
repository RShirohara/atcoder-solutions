use proconio::input;

fn main() {
  input! {
    s: String
  }

  let replaced = s
    .replace("eraser", "")
    .replace("erase", "")
    .replace("dreamer", "")
    .replace("dream", "");
  let result = if replaced == "" { "YES" } else { "NO" };

  println!("{}", result);
}
