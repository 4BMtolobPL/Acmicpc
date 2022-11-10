use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines().skip(1);
    let mut v = vec![0];
    for i in lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
    {
        v.push(i + v.last().unwrap());
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    for line in lines {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let i: usize = iter.next().unwrap();
        let j = iter.next().unwrap();
        writeln!(out, "{}", v[j] - v[i - 1]).unwrap();
    }
}
