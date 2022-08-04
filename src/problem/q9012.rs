use std::fmt::Write;
use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines: Vec<&str> = buf.lines().skip(1).collect();

    let mut out = String::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '(' {
                stack.push(c);
            } else {
                if let Some(&'(') = stack.last() {
                    stack.pop();
                } else {
                    stack.push(c);
                    break;
                }
            }
        }
        if stack.is_empty() {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }

    print!("{}", out);
}
