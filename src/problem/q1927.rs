use std::cmp::Reverse;
use std::io::Write;
use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let v: Vec<u32> = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    for value in v {
        if value == 0 {
            match heap.pop() {
                Some(x) => writeln!(out, "{}", x.0).unwrap(),
                None => writeln!(out, "0").unwrap(),
            }
        } else {
            heap.push(Reverse(value))
        }
    }
}
