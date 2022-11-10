use std::fmt::Write;
use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut out = String::new();
    let mut deque = VecDeque::new();
    for _ in 0..n {
        let order: Vec<_> = lines.next().unwrap().split_whitespace().collect();
        match order[0] {
            "push_front" => deque.push_front(order[1]),
            "push_back" => deque.push_back(order[1]),
            "pop_front" => match deque.pop_front() {
                Some(x) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "{}", -1).unwrap(),
            },
            "pop_back" => match deque.pop_back() {
                Some(x) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "{}", -1).unwrap(),
            },
            "size" => writeln!(out, "{}", deque.len()).unwrap(),
            "empty" => {
                if deque.is_empty() {
                    writeln!(out, "{}", 1).unwrap();
                } else {
                    writeln!(out, "{}", 0).unwrap();
                }
            }
            "front" => match deque.front() {
                Some(x) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "{}", -1).unwrap(),
            },
            "back" => match deque.back() {
                Some(x) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "{}", -1).unwrap(),
            },
            _ => {}
        }
    }

    print!("{}", out);
}
