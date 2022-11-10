use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let v: Vec<i32> = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let mut sorted_v = v.clone();
    sorted_v.sort_unstable();
    sorted_v.dedup();

    let mut out = String::new();
    for i in v {
        write!(out, "{} ", sorted_v.binary_search(&i).unwrap()).unwrap();
    }

    println!("{}", out.trim());
}
