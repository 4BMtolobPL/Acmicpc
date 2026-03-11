use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let k: i32 = buf.trim().parse().unwrap();

    println!("3\n1 1 1 {}", k + 1);
}
