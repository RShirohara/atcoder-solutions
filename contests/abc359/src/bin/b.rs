use proconio::input;

fn main() {
    input! {
      n: usize,
      s: [usize; n*2],
    }

    let mut res = 0;
    for color in 1..n + 1 {
        let colors = s
            .iter()
            .enumerate()
            .filter(|&x| x.1 == &color)
            .collect::<Vec<(usize, &usize)>>();
        if colors[0].0 + 2 == colors[1].0 {
            res += 1;
        }
    }
    println!("{}", res);
}
