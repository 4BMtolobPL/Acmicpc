use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let a: i64 = iter.next().unwrap();
    let b = iter.next().unwrap();

    println!("{}", (a + b) * (a - b));
}
