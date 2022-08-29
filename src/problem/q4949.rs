use std::fmt::Write;
use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let mut out = String::new();
    for line in lines {
        if ".".eq(line) {
            break;
        }
        if check_stack(line) {
            writeln!(out, "yes").unwrap();
        } else {
            writeln!(out, "no").unwrap();
        }
    }

    print!("{}", out);
}

fn check_stack(line: &str) -> bool {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push('('),
            ')' => {
                let last = match stack.last() {
                    Some(x) => x,
                    None => {
                        stack.push(')');
                        break;
                    }
                };
                if '('.eq(last) {
                    stack.pop();
                } else {
                    stack.push(')');
                    break;
                }
            }
            '[' => stack.push('['),
            ']' => {
                let last = match stack.last() {
                    Some(x) => x,
                    None => {
                        stack.push(']');
                        break;
                    }
                };
                if '['.eq(last) {
                    stack.pop();
                } else {
                    stack.push(')');
                    break;
                }
            }
            '.' => break,
            _ => continue,
        }
    }

    stack.is_empty()
}
