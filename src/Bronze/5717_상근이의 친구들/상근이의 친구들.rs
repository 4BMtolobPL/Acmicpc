use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut out = String::new();
    for line in buf.lines() {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let m: i32 = iter.next().unwrap();
        let f = iter.next().unwrap();

        if m == 0 && f == 0 {
            break;
        }

        writeln!(out, "{}", m + f).unwrap();
    }

    print!("{out}");
}
