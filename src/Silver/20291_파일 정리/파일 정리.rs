use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<&str> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.split_once('.').unwrap().1)
        .collect();
    v.sort_unstable();

    let mut iter = v.iter();
    let first = iter.next().unwrap();
    let mut prev = *first;
    let mut sum = 1;
    for &extension in iter {
        if prev == extension {
            sum += 1;
        } else {
            println!("{prev} {sum}");
            prev = extension;
            sum = 1;
        }
    }

    println!("{prev} {sum}");
}
