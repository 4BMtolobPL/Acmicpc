use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let max_weight: usize = iter.next().unwrap();

    let mut values = vec![0; max_weight + 1];

    for _ in 0..n {
        let weight: usize = iter.next().unwrap();
        let value: usize = iter.next().unwrap();

        if weight <= max_weight {
            for target_weight in (weight..=max_weight).rev() {
                values[target_weight] =
                    values[target_weight].max(values[target_weight - weight] + value);
            }
        }
    }

    println!("{}", values[max_weight]);
}
