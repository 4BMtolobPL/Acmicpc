use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let array_a: Vec<usize> = lines
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counter = vec![0; 1001];
    for i in array_a.iter() {
        counter[*i] += 1;
    }

    let mut prev = 0;
    for i in counter.iter_mut() {
        let x = prev;
        prev += *i;
        *i = x;
    }

    println!(
        "{}",
        array_a
            .iter()
            .map(|x| {
                let e = counter[*x];
                counter[*x] += 1;
                e.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}
