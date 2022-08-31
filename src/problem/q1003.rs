use std::fmt::Write;
use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    iter.next();
    let v: Vec<i32> = iter.collect();

    let mut note = vec![(1, 0), (0, 1)];

    for i in 2..=40 {
        let a = note[i - 1];
        let b = note[i - 2];
        note.push((a.0 + b.0, a.1 + b.1));
    }

    let mut out = String::new();
    for i in v {
        let x = note[i as usize];
        writeln!(out, "{} {}", x.0, x.1).unwrap();
    }
    print!("{}", out);
}
