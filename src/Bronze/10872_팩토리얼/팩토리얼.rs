use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();

    println!("{}", (1..=n).product::<i32>());
}
