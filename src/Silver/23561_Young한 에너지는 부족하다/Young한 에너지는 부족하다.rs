use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut v: Vec<i32> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    v.sort_unstable();
    println!("{}", v[(v.len() / 3) * 2 - 1] - v[v.len() / 3]);
}
