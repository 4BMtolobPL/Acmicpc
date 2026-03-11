use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut s = String::new();
    for i in 0.. {
        write!(s, "{i:b}").unwrap();
        if s.len() > n * 5 {
            break;
        }
    }

    let mut ans_iter = s.chars().skip(k - 1).step_by(n).take(5);
    let mut out = String::new();
    write!(out, "{}", ans_iter.next().unwrap()).unwrap();
    for i in ans_iter {
        write!(out, " {i}").unwrap();
    }

    println!("{out}");
}
