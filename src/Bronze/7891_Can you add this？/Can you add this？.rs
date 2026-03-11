use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let _t = lines.next().unwrap();

    let mut out = String::new();
    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let a: i32 = iter.next().unwrap();
        let b = iter.next().unwrap();

        writeln!(out, "{}", a + b).unwrap();
    }

    print!("{out}");
}
