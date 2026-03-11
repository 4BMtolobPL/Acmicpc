use std::collections::HashSet;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let _n = iter.next().unwrap();
    let m: usize = iter.next().unwrap();
    let set: HashSet<usize> = iter.collect();
    let mut v: Vec<usize> = set.into_iter().collect();
    v.sort_unstable();

    let mut stack = Vec::with_capacity(m);
    stack.push(0);
    let mut out = String::new();
    while !stack.is_empty() {
        if stack.len() == m {
            let mut iter = stack.iter();
            write!(out, "{}", v[*(iter.next().unwrap())]).unwrap();
            for i in iter {
                write!(out, " {}", v[*i]).unwrap();
            }
            writeln!(out).unwrap();

            while let Some(last) = stack.pop() {
                if last < v.len() - 1 {
                    stack.push(last + 1);
                    break;
                }
            }
        } else if let Some(&last) = stack.last() {
            for _ in 0..(m - stack.len()) {
                stack.push(last);
            }
        }
    }

    print!("{out}");
}
