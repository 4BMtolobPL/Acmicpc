use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (r, c): (usize, _) = {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    println!("{} {}", r * c / 2, r * c - 1);
}
