use std::io::Write;
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let test_case = lines.next().unwrap().parse().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    for _ in 0..test_case {
        let n = lines.next().unwrap().parse().unwrap();
        let mut cloths = HashMap::new();

        for _ in 0..n {
            let mut iter = lines.next().unwrap().split_whitespace();
            let kind = iter.nth(1).unwrap();

            let counter = cloths.entry(kind).or_insert(0);
            *counter += 1;
        }

        let mut count = 1;
        for value in cloths.values() {
            count *= value + 1;
        }

        writeln!(out, "{}", count - 1).unwrap();
    }
}
