use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let mut v: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n {
        v.push((iter.next().unwrap(), iter.next().unwrap()));
    }

    v.sort_by(|lhs, rhs| {
        if lhs.0.cmp(&rhs.0) == std::cmp::Ordering::Equal {
            lhs.1.cmp(&rhs.1)
        } else {
            lhs.0.cmp(&rhs.0)
        }
    });

    let mut out = String::new();
    for i in v {
        writeln!(out, "{} {}", i.0, i.1).unwrap();
    }

    println!("{}", out.trim());
}
