use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let (n, s): (i32, _) = (iter.next().unwrap(), iter.next().unwrap());
        println!("{}", s / (n + 1));
    }
}