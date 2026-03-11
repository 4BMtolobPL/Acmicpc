use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let mut out = String::new();
    for mut iter in lines.skip(1).map(|line| line.split_ascii_whitespace()) {
        let lhs = iter.next().unwrap();
        let rhs = iter.next().unwrap();

        let mul = mul(lhs, rhs);
        let mul2 = mul2(lhs, rhs);

        writeln!(out, "{}", if mul == mul2 { 1 } else { 0 }).unwrap();
    }

    print!("{out}");
}

fn mul(lhs: &str, rhs: &str) -> i64 {
    lhs.parse::<i64>().unwrap() * rhs.parse::<i64>().unwrap()
}

fn mul2(lhs: &str, rhs: &str) -> i64 {
    let (mut long, mut short, len) = if lhs.len() >= rhs.len() {
        (lhs.chars().rev(), rhs.chars().rev(), lhs.len())
    } else {
        (rhs.chars().rev(), lhs.chars().rev(), rhs.len())
    };

    let mut result = Vec::with_capacity(len);

    for _ in 0..len {
        if let Some(l) = long.next() {
            if let Some(s) = short.next() {
                result.push((l.to_digit(10).unwrap() * s.to_digit(10).unwrap()).to_string());
            } else {
                result.push(l.to_string());
            }
        }
    }

    result.reverse();
    result.join("").parse().unwrap()
}
