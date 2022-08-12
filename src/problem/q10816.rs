use std::{io::{stdin, Read}, collections::HashMap};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    lines.next();
    let cards = lines.next().unwrap();
    lines.next();
    let targets = lines.next().unwrap();

    let mut card_map: HashMap<&str, i32> = HashMap::new();
    for i in cards.split_whitespace() {
        let counter = card_map.entry(i).or_insert(0);
        *counter += 1;
    }
    
    let mut v = Vec::new();
    for i in targets.split_whitespace() {
        let r = match card_map.get(i) {
            Some(x) => x.to_string(),
            None => "0".to_string(),
        };
        v.push(r);
    }

    println!("{}", v.join(" "));
}