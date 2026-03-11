use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut answers = String::new();

    let mut stack: Vec<_> = vec![1; m];

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
            if last < n {
                last += 1;
                for _ in 0..(m - stack.len()) {
                    stack.push(last);
                }
            }
        } else {
            break;
        }
    }

    print!("{answers}");
}
