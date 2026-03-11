use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let v: Vec<char> = lines
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();

    let c = v.first().unwrap();

    println!("{}", if v.iter().all(|x| x.eq(c)) { 1 } else { 0 });
}
