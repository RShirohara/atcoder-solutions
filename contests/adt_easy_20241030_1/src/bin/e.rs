use itertools::sorted;
use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
      a: [usize; n],
      x: [usize; q],
    }

    let heights: Vec<usize> = sorted(a.clone()).collect();
    for j in x.iter() {
        let mut ok: isize = isize::try_from(n).unwrap();
        let mut ng: isize = -1;
        while (ok - ng) > 1 {
            let md = (ok + ng) / 2;
            let x = md as usize;
            if j <= &heights[x] {
                ok = md
            } else {
                ng = md
            }
        }
        println!("{}", (n as isize) - ok)
    }
}
