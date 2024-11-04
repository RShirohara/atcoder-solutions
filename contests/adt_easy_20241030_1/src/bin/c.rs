use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [[usize; n]; n],
    b: [[usize; n]; n],
  }

  let mut rotated = a.clone();
  for _ in 0..4 {
    let mut ok = true;

    for i in 0..rotated.len() {
      for j in 0..rotated[i].len() {
        if (rotated[i][j] == 1) && (b[i][j] == 0) {
          ok = false
        }
      }
    }

    if ok {
      println!("Yes");
      return;
    }
    rotated = rotate(&rotated, &n);
  }

  println!("No");
}

fn rotate(a: &Vec<Vec<usize>>, n: &usize) -> Vec<Vec<usize>> {
  let mut rotated = vec![vec![0; *n]; *n];
  for i in 0..a.len() {
    for j in 0..a[i].len() {
      rotated[j][n - 1 - i] = a[i][j];
    }
  }
  rotated
}
