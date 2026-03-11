use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut out = String::new();
    for line in buf.lines() {
        let n: i32 = line.trim_end().parse().unwrap();

        if n == 0 {
            break;
        }

        writeln!(out, "{}", (1 + n) * n / 2).unwrap();
    }

    print!("{out}");
}
