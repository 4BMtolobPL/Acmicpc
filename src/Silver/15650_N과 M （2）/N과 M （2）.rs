use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut answers = String::new();

    let mut stack: Vec<_> = Vec::new();
    for i in 1..=m {
        stack.push(i);
    }

    loop {
        if stack.len() == m {
            let mut iter = stack.iter();
            write!(answers, "{}", iter.next().unwrap()).unwrap();
            for i in iter {
                write!(answers, " {i}").unwrap();
            }
            writeln!(answers).unwrap();
        }

        if let Some(mut last) = stack.pop() {
            if m - stack.len() + last <= n {
                last += 1;
                for i in (last..).take(m - stack.len()) {
                    stack.push(i);
                }
            }
        } else {
            break;
        }
    }

    print!("{answers}");
}
