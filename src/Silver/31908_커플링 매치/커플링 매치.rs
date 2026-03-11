use std::collections::HashMap;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let mut map = HashMap::new();

    lines.skip(1).for_each(|line| {
        let (name, ring) = line.trim().split_once(" ").unwrap();
        if ring != "-" {
            map.entry(String::from(ring))
                .or_insert(Vec::new())
                .push(String::from(name));
        }
    });

    let couples: Vec<_> = map.values().filter(|x| x.len() == 2).collect();
    let mut out = String::new();
    writeln!(out, "{}", couples.len()).unwrap();
    for couple in couples {
        writeln!(out, "{}", couple.join(" ")).unwrap();
    }

    println!("{}", out);
}
