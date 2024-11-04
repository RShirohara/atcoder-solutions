use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    a: [usize; n],
    x: [usize; q],
  }

  let results = x
    .iter()
    .map(|j| a.iter().filter(|&i| j <= i).collect::<Vec<&usize>>().len())
    .collect::<Vec<usize>>();

  for r in results {
    println!("{}", r)
  }
}
