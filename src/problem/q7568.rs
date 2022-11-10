use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let v: Vec<(i16, i16)> = lines
        .map(|x| {
            let mut a = x.split_whitespace().map(|y| y.parse().unwrap());
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect();

    let mut out = String::new();

    for index in 0..n {
        let (weight, height) = v[index];
        let count = v.iter().filter(|(w, h)| *w > weight && *h > height).count();
        write!(out, "{} ", count + 1).unwrap();
    }

    println!("{}", out.trim());
}
