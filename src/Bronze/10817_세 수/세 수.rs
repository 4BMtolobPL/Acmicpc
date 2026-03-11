use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut v: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    v.sort_unstable();

    println!("{}", v[1]);
}
