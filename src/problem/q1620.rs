use std::io::Write;
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut iter = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    iter.next();

    let mut hashmap_by_num = HashMap::new();
    let mut hashmap_by_name = HashMap::new();
    for key in 1..=n {
        let name = lines.next().unwrap();
        hashmap_by_num.insert(key, name);
        hashmap_by_name.insert(name, key);
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    for line in lines {
        match line.parse::<i32>() {
            Ok(key) => {
                writeln!(out, "{}", hashmap_by_num.get(&key).unwrap()).unwrap();
            }
            Err(_) => {
                writeln!(out, "{}", hashmap_by_name.get(line).unwrap()).unwrap();
            }
        }
    }
}
