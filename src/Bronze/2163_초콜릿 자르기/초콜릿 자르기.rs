use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mul: i32 = buf
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .reduce(|acc, e| acc * e)
        .unwrap();
    println!("{}", mul - 1);
}
