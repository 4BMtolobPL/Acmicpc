use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    lines.next();

    let mut v: Vec<(i32, i32)> = lines
        .map(|x| {
            let mut a = x.split_whitespace().map(|y| y.parse().unwrap());
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect();

    v.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut out = String::new();
    for (x, y) in v {
        writeln!(out, "{} {}", x, y).unwrap();
    }

    print!("{}", out);
}
