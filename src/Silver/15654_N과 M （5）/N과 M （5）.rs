use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let (n, m) = { (iter.next().unwrap(), iter.next().unwrap()) };
    let mut v: Vec<_> = iter.collect();
    v.sort();

    let mut answers = String::new();

    let mut stack: Vec<_> = Vec::new();
    stack.push(0);

    while !stack.is_empty() {
        if stack.len() == m {
            let mut iter = stack.iter();
            write!(answers, "{}", v[*iter.next().unwrap()]).unwrap();
            for i in iter {
                write!(answers, " {}", v[*i]).unwrap();
            }
            writeln!(answers).unwrap();

            while let Some(last) = stack.pop() {
                let mut next = last + 1;

                while stack.contains(&next) {
                    next += 1;
                }

                if next < n {
                    stack.push(next);
                    break;
                }
            }
        } else {
            for i in 0..m {
                if !stack.contains(&i) {
                    stack.push(i);
                    break;
                }
            }
        }
    }

    print!("{answers}");
}
