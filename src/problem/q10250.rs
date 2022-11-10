use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let cases: Vec<Vec<i32>> = buf
        .lines()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let mut out = String::new();

    for case in cases {
        let mut room = case[2] % case[0] * 100 + case[2] / case[0] + 1;
        if room < 100 {
            room += case[0] * 100;
            room -= 1;
        }
        writeln!(out, "{}", room).unwrap();
    }

    print!("{}", out);
}
