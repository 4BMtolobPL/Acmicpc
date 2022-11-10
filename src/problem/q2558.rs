use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!(
        "{}",
        buf.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
