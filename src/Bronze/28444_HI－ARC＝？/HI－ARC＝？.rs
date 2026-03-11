use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();
    let mut iter = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let a: i32 = iter.by_ref().take(2).product();
    let b: i32 = iter.by_ref().take(3).product();

    println!("{}", a - b);
}
