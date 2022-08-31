use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut table = [0; 11];
    table[1] = 1;
    table[2] = 2;
    table[3] = 4;

    for index in 4..table.len() {
        table[index] = table[index - 1] + table[index - 2] + table[index - 3];
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    let iter = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());
    for i in iter {
        writeln!(out, "{}", table[i]).unwrap();
    }
}
