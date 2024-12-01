use itertools::sorted;
use proconio::input;

fn main() {
    input! {
      n: usize,
      d: [usize; n]
    }

    let mut count = 0;
    let mut before_x = 101;

    for x in sorted(d.iter()).rev() {
        if x < &before_x {
            count += 1;
        }
        before_x = x.to_owned();
    }

    println!("{}", count);
}
