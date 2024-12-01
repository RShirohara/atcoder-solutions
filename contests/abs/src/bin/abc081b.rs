use proconio::input;

fn main() {
    input! {
      n: u64,
      a: [u64; n]
    }
    let mut process_cnt = 0;
    let mut divide = a.to_owned();

    'main: loop {
        for val in divide.iter_mut() {
            if *val % 2 == 1 {
                break 'main;
            }
            *val /= 2;
        }
        process_cnt += 1
    }

    println!("{}", process_cnt);
}
