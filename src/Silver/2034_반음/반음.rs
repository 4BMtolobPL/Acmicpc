use std::collections::HashMap;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let score: Vec<i32> = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let solfa: HashMap<i32, char> = HashMap::from([
        (0, 'A'),
        (2, 'B'),
        (3, 'C'),
        (5, 'D'),
        (7, 'E'),
        (8, 'F'),
        (10, 'G'),
    ]);

    'outer: for start in [0, 2, 3, 5, 7, 8, 10] {
        let mut position = start;

        for d in score.iter() {
            let new_position = (position + d + 24) % 12;
            if solfa.contains_key(&new_position) {
                position = new_position
            } else {
                continue 'outer;
            }
        }

        println!(
            "{} {}",
            solfa.get(&start).unwrap(),
            solfa.get(&position).unwrap()
        );
    }
}
