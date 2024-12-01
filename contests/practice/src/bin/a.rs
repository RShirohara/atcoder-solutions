use proconio::input;

fn main() {
    input! {
      a: i32,
      b: i32,
      c: i32,
      s: String
    }
    println!("{sum} {s}", sum = a + b + c, s = s);
}
