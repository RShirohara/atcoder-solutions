use proconio::input;

fn main() {
  input! {
    n: usize,
    p: usize,
    q: usize,
    d: [usize; n],
  }

  let minimum_menu_amount = d.iter().min().unwrap();
  let discounted_amount = minimum_menu_amount + q;
  let result = match p < discounted_amount {
    true => p,
    false => discounted_amount,
  };

  println!("{}", result)
}
