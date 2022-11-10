use std::io::Write;
use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    let mut heap: BinaryHeap<ReverseAbsolute> = BinaryHeap::new();
    for i in buf.split_whitespace().map(|x| x.parse().unwrap()).skip(1) {
        match i {
            0 => match heap.pop() {
                Some(x) => writeln!(out, "{}", x.0).unwrap(),
                None => writeln!(out, "0").unwrap(),
            },
            _ => heap.push(ReverseAbsolute(i)),
        }
    }
}

#[derive(PartialEq, Eq)]
struct ReverseAbsolute(i32);

impl PartialOrd for ReverseAbsolute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = if self.0.is_positive() {
            self.0
        } else {
            -self.0
        };
        let rhs = if other.0.is_positive() {
            other.0
        } else {
            -other.0
        };
        if let Some(x) = rhs.partial_cmp(&lhs) {
            match x {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => other.0.partial_cmp(&self.0),
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            }
        } else {
            None
        }
    }
}

impl Ord for ReverseAbsolute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = if self.0.is_positive() {
            self.0
        } else {
            -self.0
        };
        let rhs = if other.0.is_positive() {
            other.0
        } else {
            -other.0
        };
        match rhs.cmp(&lhs) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => other.cmp(self),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}
