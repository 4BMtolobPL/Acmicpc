use std::io::Write;
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut iter = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    iter.next();

    let mut store = HashMap::new();
    for _ in 0..n {
        let mut words = lines.next().unwrap().split_whitespace();
        store.insert(words.next().unwrap(), words.next().unwrap());
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    for line in lines {
        writeln!(out, "{}", store.get(line).unwrap()).unwrap();
    }
}
