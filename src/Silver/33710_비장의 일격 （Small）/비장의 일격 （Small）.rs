use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let (n, _k): (usize, _) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    let s = lines.next().unwrap().trim();

    let mut char_position_map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        (*char_position_map.entry(c).or_default()).push(i);
    }

    if char_position_map.contains_key(&'X') {
        char_position_map.remove(&'X');
    }

    let mut range_array: Vec<(usize, usize, usize)> = Vec::new();
    for value in char_position_map.values() {
        for (i, lhs) in value.iter().enumerate() {
            for rhs in value.iter().skip(i + 1) {
                range_array.push((*lhs, *rhs, rhs - lhs + 1));
            }
        }
    }

    range_array.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.1.cmp(&b.1),
        Ordering::Greater => Ordering::Greater,
    });

    let mut max = 0;
    for (i, l) in range_array.iter().enumerate() {
        max = max.max(l.2);

        for j in range_array.iter().skip(i) {
            if l.1 >= j.0 {
                continue;
            }
            max = max.max(l.2 + j.2);
        }
    }

    println!("{}", n - max);
}
