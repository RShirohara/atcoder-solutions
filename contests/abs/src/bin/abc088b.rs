use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n]
  }

  let mut cards = a.to_owned();
  cards.sort();
  cards.reverse();
  let mut alices_score = 0;
  for (i, card) in cards.iter().enumerate() {
    if i % 2 == 0 {
      alices_score += card;
    } else {
      alices_score -= card
    }
  }

  println!("{}", alices_score);
}
