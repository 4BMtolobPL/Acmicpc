use std::collections::HashMap;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    println!("{}", is_normal(lines.nth(1).unwrap()));
}

fn is_normal(deck: &str) -> i32 {
    let mut card_map: HashMap<String, i32> = HashMap::new();
    let mut count = 0;

    for i in deck.split_whitespace() {
        count += 1;
        *card_map.entry(i.to_string()).or_insert(0) += 1;
        if *card_map.get(i).unwrap() > 4 {
            return count;
        }
    }

    0
}
