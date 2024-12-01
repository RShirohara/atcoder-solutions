use num::abs;
use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [[isize; 3]; n]
    }

    let mut can_travel = true;
    let (mut prev_t, mut prev_x, mut prev_y) = (0, 0, 0);

    for _a in a.iter() {
        let t = _a[0];
        let x = _a[1];
        let y = _a[2];
        let dist = abs(x - prev_x) + abs(y - prev_y);
        let time = t - prev_t;

        if dist > time || (time - dist) % 2 != 0 {
            can_travel = false;
            break;
        }
        (prev_t, prev_x, prev_y) = (t, x, y);
    }

    println!("{}", if can_travel { "Yes" } else { "No" });
}
