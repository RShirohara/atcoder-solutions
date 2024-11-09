use proconio::input;

fn main() {
  input! {
    w: u8,
    b: u8
  }
  const T: &str = "wbwbwwbwbwbw";
  let mut result = "No";

  for i in 0..T.len() {
    let mut nw: u8 = 0;
    let mut nb: u8 = 0;

    for j in 0..(w+b) {
      let index = (i as usize + j as usize) % T.len();
      if T.chars().nth(index).unwrap() == 'w' {
        nw += 1;
      } else {
        nb += 1;
      }
    }

    if w == nw && b == nb {
      result = "Yes";
      break;
    }
  }

  println!("{}", result);
}
