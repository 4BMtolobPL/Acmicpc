use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let s = iter.next().unwrap();
    let i = iter.next().unwrap().parse::<usize>().unwrap() - 1;

    println!("{}", s.chars().nth(i).unwrap());
}
