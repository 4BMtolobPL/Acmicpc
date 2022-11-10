use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let iter = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let mut progression: Vec<i64> = vec![0, 1, 1, 1, 2, 2, 3, 4, 5, 7, 9];
    for index in 11..=100 {
        progression.push(progression[index - 1] + progression[index - 5]);
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    for i in iter {
        writeln!(out, "{}", progression[i]).unwrap();
    }
}
