use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; 4*n-1],
  }

  let mut numbers = vec![0; n];
  for i in a {
    numbers[i - 1] += 1;
  }

  let result = numbers
    .iter()
    .enumerate()
    .min_by(|a, b| a.1.cmp(b.1))
    .unwrap()
    .0+1;

  println!("{}", result)
}
