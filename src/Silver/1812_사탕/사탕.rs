use std::fmt::Write;
use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();

    let v: Vec<i32> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut prev =
        (v.iter().step_by(2).sum::<i32>() - v.iter().skip(1).step_by(2).sum::<i32>()) / 2;
    let mut out = String::new();
    for i in v.iter() {
        writeln!(out, "{} ", prev).unwrap();
        prev = *i - prev;
    }

    print!("{}", out);
}
