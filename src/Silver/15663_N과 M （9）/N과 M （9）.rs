use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let m = iter.next().unwrap();
    let mut v: Vec<_> = iter.collect();
    v.sort_unstable();

    let mut out = String::new();

    let mut stack = vec![0];
    while !stack.is_empty() {
        if stack.len() < m {
            for i in 0..n {
                if !stack.contains(&i) {
                    stack.push(i);
                    break;
                }
            }
        } else {
            let mut iter = stack.iter();
            write!(out, "{}", v[*iter.next().unwrap()]).unwrap();
            for i in iter {
                write!(out, " {}", v[*i]).unwrap();
            }
            writeln!(out).unwrap();

            'outer: while let Some(last) = stack.pop() {
                for i in (last + 1)..n {
                    if v[last] == v[i] {
                        continue;
                    }
                    if !stack.contains(&i) {
                        stack.push(i);
                        break 'outer;
                    }
                }
            }
        }
    }

    print!("{out}");
}
