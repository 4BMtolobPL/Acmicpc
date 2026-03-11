use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let map: HashSet<String> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.to_string())
        .collect();

    for w in map.iter() {
        if map.contains(&w.chars().rev().collect::<String>()) {
            println!("{} {}", w.len(), w.chars().nth(w.len() / 2).unwrap());
            break;
        }
    }
}
