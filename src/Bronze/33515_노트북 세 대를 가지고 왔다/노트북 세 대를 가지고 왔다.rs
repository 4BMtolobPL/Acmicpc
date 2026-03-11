use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let min: i32 = buf
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .min()
        .unwrap();

    println!("{min}");
}
