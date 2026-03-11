use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut array: Vec<i32> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let v = array.pop().unwrap();

    println!("{}", array.iter().filter(|x| **x == v).count())
}
