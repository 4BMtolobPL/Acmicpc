use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let v: Vec<(i32, i32, i32)> = buf
        .lines()
        .skip(1)
        .map(|x| {
            let mut iter = x.split_whitespace().map(|x| x.parse().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();

    let mut sum = (0, 0, 0);

    for value in v {
        let x = value.0 + sum.1.min(sum.2);
        let y = value.1 + sum.0.min(sum.2);
        let z = value.2 + sum.0.min(sum.1);
        sum = (x, y, z);
    }

    println!("{}", sum.0.min(sum.1.min(sum.2)));
}