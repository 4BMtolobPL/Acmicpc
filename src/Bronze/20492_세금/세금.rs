use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    println!("{} {}", n / 100 * 78, n / 250 * 239)
}
