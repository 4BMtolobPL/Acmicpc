use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!(
        "{}",
        buf.split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .sum::<u64>()
    );
}
