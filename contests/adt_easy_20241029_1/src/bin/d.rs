use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
      event: [(usize, usize); q]
    }

    let mut penalty = vec![0; n];

    for i in 0..q {
        match event[i].0 {
            1 => penalty[event[i].1 - 1] += 1,
            2 => penalty[event[i].1 - 1] += 2,
            3 => match penalty[event[i].1 - 1] >= 2 {
                true => println!("Yes"),
                false => println!("No"),
            },
            _ => todo!(),
        }
    }
}
