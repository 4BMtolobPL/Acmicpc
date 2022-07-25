use std::{io::{stdin, Read}, collections::HashMap};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut numbers: Vec<i32> = buf.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    numbers.sort();

    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;

    let mid = numbers[numbers.len() / 2];

    let mut map = HashMap::new();
    for i in numbers.iter() {
        let counter = map.entry(*i).or_insert(0);
        *counter += 1;
    }
    let max_value = *map.values().max().unwrap();
    let mut maxs = Vec::new();
    for (key, value) in map {
        if value == max_value {
            maxs.push(key);
        }
    }

    let most = if maxs.len() == 1 {
        maxs.pop().unwrap()
    } else {
        maxs.sort();
        maxs[1]
    };

    let range = numbers.iter().max().unwrap() - numbers.iter().min().unwrap();

    println!("{}\n{}\n{}\n{}", avg.round() as i32, mid, most, range);
}