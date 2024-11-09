use proconio::input;

fn main() {
  input! {
    n: u32,
    l: u32,
    r: u32,
    a: [u32; n],
  }

  for i in 0..n {
    let result = if a[i as usize] < l {
      l
    } else if a[i as usize] > r {
      r
    } else {
      a[i as usize]
    };
    print!("{} ", result);
  }
  print!("\n");
}
