use proconio::input;

fn main() {
  input! {
    n: u32,
    ab: [(u16, u16); n],
  }

  let mut t: f64 = 0.0;
  let mut ans: f64 = 0.0;

  for i in 0..n {
    t += ab[i as usize].0 as f64 / ab[i as usize].1 as f64;
  }
  t /= 2.0;
  for i in 0..n {
    ans += (ab[i as usize].0 as f64).min(t * ab[i as usize].1 as f64);
    t -= t.min(ab[i as usize].0 as f64 / ab[i as usize].1 as f64);
  }

  println!("{}", ans)
}
