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
    let mut list = VecDeque::new();
    let mut answer = Vec::new();

    for i in 1..=n {
        list.push_back(i);
    }

    while !list.is_empty() {
        for _ in 0..(k - 1) {
            let front_element = list.pop_front().unwrap();
            list.push_back(front_element);
        }
        answer.push(list.pop_front().unwrap().to_string());
    }

    println!("<{}>", answer.join(", "));
}
