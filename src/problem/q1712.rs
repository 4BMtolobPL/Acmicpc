use std::io::Write;
use std::io::{stdin, stdout, BufWriter};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let (a, b, c): (i32, i32, i32) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    if b >= c {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{}", a / (c - b) + 1).unwrap();
    }
}
