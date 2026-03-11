use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let mut v = vec![1, 1, 2, 2, 4, 4, 6, 6, 10, 10, 14];

    for i in iter.skip(1) {
        for j in v.len()..=i {
            v.push(v[0..=(j / 2)].iter().sum());
        }
        println!("{}", v[i]);
    }
}
