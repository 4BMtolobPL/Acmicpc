use std::collections::HashMap;
use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();

    let mut iter = buf.split_ascii_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(n);
    for name in iter {
        *map.entry(name).or_insert(0) += 1;
    }

    println!("{}", map.iter().find(|(_, v)| **v % 2 == 1).unwrap().0);
}
