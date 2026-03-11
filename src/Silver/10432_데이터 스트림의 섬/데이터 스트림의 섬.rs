use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let vs: Vec<Vec<i32>> = buf
        .lines()
        .skip(1)
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let mut out = String::new();
    for (vi, v) in vs.iter().enumerate() {
        let mut count = 0;
        for (lhs, value_l) in v.iter().enumerate() {
            for (rhs, value_r) in v.iter().enumerate().skip(lhs + 2) {
                if v[lhs + 1..rhs]
                    .iter()
                    .all(|x| *value_l < *x && *value_r < *x)
                {
                    count += 1;
                }
            }
        }

        writeln!(out, "{} {count}", vi + 1).unwrap();
    }

    print!("{out}");
}
