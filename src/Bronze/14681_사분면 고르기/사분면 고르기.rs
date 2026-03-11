use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let x: i32 = iter.next().unwrap();
    let y = iter.next().unwrap();

    let n = if x > 0 && y > 0 {
        1
    } else if x < 0 && y > 0 {
        2
    } else if x < 0 && y < 0 {
        3
    } else {
        4
    };

    println!("{n}");
}
