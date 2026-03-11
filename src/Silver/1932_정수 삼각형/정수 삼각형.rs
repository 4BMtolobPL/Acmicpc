use std::{io::{stdin, Read}, vec};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut v = vec![0];

    for (index, line) in buf.lines().skip(1).enumerate() {
        let mut t: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for (n, i) in t.iter_mut().enumerate() {
            if n == 0 {
                *i += v[0];
            } else if n == index {
                *i += v[n - 1];
            } else {
                *i += v[n - 1].max(v[n]);
            }
        }
        v = t;
    }

    println!("{}", v.iter().max().unwrap());
}