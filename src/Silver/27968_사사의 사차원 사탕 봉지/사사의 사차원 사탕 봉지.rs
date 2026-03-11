use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let mut v: Vec<i64> = vec![0];
    let mut sum = 0;
    for i in lines
        .by_ref()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
    {
        sum += i;
        v.push(sum);
    }

    let mut out = String::new();
    for line in lines {
        let b = line.parse().unwrap();

        let point = v.partition_point(|x| *x < b);
        if point == v.len() {
            writeln!(out, "Go away!").unwrap();
        } else {
            writeln!(out, "{point}").unwrap();
        }
    }

    print!("{out}");
}
