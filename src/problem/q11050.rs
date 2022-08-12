use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n:i32 = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut head = 1;
    for i in (k + 1)..=n {
        head *= i;
    }

    let mut tail = 1;
    for i in 1..=(n - k) {
        tail *= i;
    }

    println!("{}", head / tail);
}