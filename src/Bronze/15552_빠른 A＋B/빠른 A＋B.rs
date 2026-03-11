use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();

    let mut out = String::new();
    for _ in 0..n {
        writeln!(out, "{}", iter.next().unwrap() + iter.next().unwrap()).unwrap();
    }

    print!("{out}");
}
