use std::fmt::Write;
use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let v: Vec<Vec<i32>> = buf
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut out = String::new();

    for mut i in v {
        if i[0] == 0 && i[1] == 0 && i[2] == 0 {
            break;
        }
        i.sort();
        if i[0] * i[0] + i[1] * i[1] == i[2] * i[2] {
            writeln!(out, "right").unwrap();
        } else {
            writeln!(out, "wrong").unwrap();
        }
    }

    print!("{}", out);
}