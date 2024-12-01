use proconio::input;

fn main() {
    input! {
      n: usize,
      a: usize,
      b: usize
    }
    let mut sum = 0;

    for n in 1..n + 1 {
        let n_string = n.to_string();
        let n_sum = n_string
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .fold(0, |sum, x| sum + x);
        if n_sum >= a && n_sum <= b {
            sum += n;
        }
    }

    println!("{}", sum);
}
