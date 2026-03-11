use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    println!(
        "{}\n{}",
        a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap() - c,
        [a, b].concat().parse::<i32>().unwrap() - c
    );
}
