use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    let mut idiot_man = vec![0; 3];
    idiot_man[a - 1] = 1;
    idiot_man[b - 1] = 1;

    let idiot_index = idiot_man
        .iter()
        .enumerate()
        .filter(|(_, &v)| v != 1)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let result: isize = match idiot_index.len() == 1 {
        true => isize::try_from(idiot_index[0] + 1).unwrap(),
        false => -1,
    };

    println!("{}", result)
}
