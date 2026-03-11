use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    for line in buf.lines() {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let (lhs, rhs): (i32, _) = (iter.next().unwrap(), iter.next().unwrap());
        if lhs == 0 && rhs == 0 {
            break;
        }
        writeln!(out, "{}", if lhs > rhs { "Yes" } else { "No" }).unwrap();
    }
}