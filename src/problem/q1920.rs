use std::{
    collections::HashSet,
    io::{prelude::*, stdin},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    lines.next();
    let a: HashSet<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();
    let v: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut out = String::new();

    for i in v {
        if a.contains(&i) {
            out.push_str("1\n");
        } else {
            out.push_str("0\n");
        }
    }

    println!("{}", out.trim());
}
