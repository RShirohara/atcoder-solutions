use num::ToPrimitive;
use proconio::input;

fn main() {
  input! {
    n: usize,
    yen: usize
  }

  let (mut x, mut y, mut z): (isize, isize, isize) = (-1, -1, -1);
  'main: for _x in 0..=n {
    for _y in 0..=n - _x {
      let _z = n - _x - _y;
      let total = 10000 * _x + 5000 * _y + 1000 * _z;
      if total == yen {
        (x, y, z) = (
          _x.to_isize().unwrap(),
          _y.to_isize().unwrap(),
          _z.to_isize().unwrap(),
        );
        break 'main;
      }
    }
  }

  println!("{} {} {}", x, y, z);
}
