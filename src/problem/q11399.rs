use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<i32> = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    v.sort_unstable();
    let mut sum = 0;
    let mut s = Vec::new();
    for i in v {
        sum += i;
        s.push(sum);
    }
    println!("{}", s.iter().sum::<i32>());
}
