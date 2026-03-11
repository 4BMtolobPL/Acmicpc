use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let k = iter.next().unwrap();

    josephus(n, k);
}

fn josephus(n: usize, k: usize) {
    let mut list: VecDeque<usize> = (1..=n).collect();
    let mut answer = Vec::new();

    let mut cursor = 0;
    while !list.is_empty() {
        cursor = (cursor + k - 1) % list.len();
        answer.push(list.remove(cursor).unwrap().to_string());
    }

    println!("<{}>", answer.join(", "));
}