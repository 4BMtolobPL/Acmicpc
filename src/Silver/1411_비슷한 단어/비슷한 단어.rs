use std::collections::HashMap;
use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();

    let iter = buf.split_ascii_whitespace();
    let mut map = HashMap::with_capacity(100);

    for word in iter.skip(1) {
        let mut count = 0;

        let mut order_map: [Option<i32>; 26] = [None; 26];
        let mut v: Vec<i32> = Vec::with_capacity(word.len());
        for i in 0..word.len() {
            if let Some(order) = order_map[(word.as_bytes()[i] - b'a') as usize] {
                v.push(order);
            } else {
                order_map[(word.as_bytes()[i] - b'a') as usize] = Some(count);
                v.push(count);
                count += 1;
            }
        }

        *map.entry(v).or_insert(0) += 1;
    }

    println!("{}", map.values().map(|x| (x) * (x - 1) / 2).sum::<i32>());
}
